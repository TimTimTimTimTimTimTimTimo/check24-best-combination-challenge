use std::{collections::HashMap, fs::File, io::BufWriter, path::Path};

use derive_more::derive::Display;
use fehler::throws;
use index_vec::{IndexSlice, IndexVec};
use itertools::Itertools;
use serde::{de::DeserializeOwned, Deserialize, Deserializer, Serialize};
use smol_str::SmolStr;
use time::{Date, PrimitiveDateTime};

use crate::Bitmap;

#[derive(Serialize, Deserialize)]
pub struct Data {
    /// Unique teams sorted descending by number of apperances in the games dataset.
    pub teams: IndexVec<TeamId, Team>,
    /// Unique tournaments
    pub tournaments: IndexVec<TournamentId, Tournament>,
    /// Games with streaming offers
    pub games: IndexVec<GameId, Game>,
    /// Games with no streaming offers
    pub orphan_games: IndexVec<OrphanGameId, OrphanGame>,
    pub packages: IndexVec<PackageId, Package>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Game {
    pub id: GameId,
    pub attributes: GameAttributes,
    /// Bitmap, where the index of each set bit indicates that the game can be watched live
    /// using the package whose id is equal to the index.
    pub live_map: u64,
    /// Bitmap, where the index of each set bit indicates that the game can be watched via highlights
    /// using the package whose id is equal to the index.
    pub high_map: u64,
}
index_vec::define_index_type! {
    pub struct GameId = u16;
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OrphanGame {
    pub id: OrphanGameId,
    pub attributes: GameAttributes,
}
index_vec::define_index_type! {
    pub struct OrphanGameId = u16;
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GameAttributes {
    pub team_home_id: TeamId,
    pub team_away_id: TeamId,
    pub date: Date,
    pub tournament_id: TournamentId,
}

impl GameAttributes {
    fn from_token(
        token: &GameToken,
        teams: &IndexSlice<TeamId, [Team]>,
        tournaments: &IndexSlice<TournamentId, [Tournament]>,
    ) -> Self {
        GameAttributes {
            team_home_id: Team(token.team_home.clone()).get_id(teams),
            team_away_id: Team(token.team_away.clone()).get_id(teams),
            date: token.starts_at.date(),
            tournament_id: Tournament(token.tournament_name.clone()).get_id(tournaments),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Package {
    pub id: PackageId,
    pub name: SmolStr,
    pub monthly_price_cents: Option<u32>,
    pub monthly_price_yearly_subscription_cents: u32,
}

index_vec::define_index_type! {
    pub struct PackageId = u8;
}

#[derive(Debug, Serialize, Deserialize, Clone, Display, PartialEq, Eq, Hash)]
pub struct Team(pub SmolStr);
index_vec::define_index_type! {
    pub struct TeamId = u16;
}
impl Team {
    fn get_id(&self, teams: &IndexSlice<TeamId, [Team]>) -> TeamId {
        TeamId::new(
            teams
                .iter()
                .position(|t| t == self)
                .expect("Team not found"),
        )
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Display, PartialEq, Eq)]
pub struct Tournament(pub SmolStr);
impl Tournament {
    fn get_id(&self, tournaments: &IndexSlice<TournamentId, [Tournament]>) -> TournamentId {
        TournamentId::new(
            tournaments
                .iter()
                .position(|t| t == self)
                .expect("Tournament not found"),
        )
    }
}
index_vec::define_index_type! {
    pub struct TournamentId = u8;
}

impl Data {
    /// Write the entire Data data structure into a binary format
    #[throws(anyhow::Error)]
    pub fn store_as_bin_file(&self, path: &Path) {
        let file = File::create(path)?;
        bincode::serialize_into(file, &self)?;
    }

    /// Write a Typescript file which contains string literal enums for teams and tournaments
    /// and all packages and games
    #[throws(anyhow::Error)]
    pub fn generate_ts_types(&self, path: &Path) {
        use std::io::Write;

        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        writeln!(
            writer,
            "
            export const teams = [{}];
            export type Team = typeof teams[number];
            ",
            self.teams
                .iter()
                .map(|team| format!("\"{team}\""))
                .join(",")
        )?;

        writeln!(writer)?;

        writeln!(
            writer,
            "
            export const tournaments = [{}];
            export type Tournament = typeof tournaments[number];
            ",
            self.tournaments
                .iter()
                .map(|tournament| format!("\"{tournament}\""))
                .join(",")
        )?;

        writeln!(writer)?;

        writeln!(
            writer,
            "
            export type Package = {{
              id: number;
              name: string;
              monthly_price_cents: number | null;
              monthly_price_yearly_subscription_cents: number;
            }}
            "
        )?;

        writeln!(
            writer,
            "export const packages: Package[] = [{}];",
            self.packages
                .iter()
                .map(|package| serde_json::to_string(&package).unwrap())
                .join(",")
        )?;

        writeln!(
            writer,
            "
            export type GameAttributes = {{
              team_home_id: number;
              team_away_id: number;
              date: string;
              tournament_id: number;
            }}
            "
        )?;

        writeln!(
            writer,
            "
            export type Game = {{
              id: number;
              attributes: GameAttributes;
              live_map: number;
              high_map: number;
            }}
            "
        )?;

        writeln!(
            writer,
            "
            export type OrphanGame = {{
              id: number;
              attributes: GameAttributes;
            }}
            "
        )?;

        writeln!(
            writer,
            "export const games: Game[] = [{}];",
            self.games
                .iter()
                .map(|game| serde_json::to_string(&game).unwrap())
                .join(",")
        )?;

        writeln!(
            writer,
            "export const orphan_games: OrphanGame[] = [{}];",
            self.orphan_games
                .iter()
                .map(|orphan_game| serde_json::to_string(&orphan_game).unwrap())
                .join(",")
        )?;

        writer.flush()?;
    }

    /// Parse the data from the given paths, then process them into a denser format more suitable for calculating combinations.
    #[throws(anyhow::Error)]
    pub fn load_from_csv(games_path: &Path, offers_path: &Path, packages_path: &Path) -> Data {
        let offer_tokens = OfferToken::parse_items_from_csv(offers_path)?;
        let (game_tokens, orphan_game_tokens): (Vec<GameToken>, Vec<GameToken>) =
            GameToken::parse_items_from_csv(games_path)?
                .iter()
                .cloned()
                .partition(|gt| offer_tokens.iter().any(|ot| ot.game_id == gt.id));
        let package_tokens = PackageToken::parse_items_from_csv(packages_path)?;

        let teams = Self::collect_teams(&game_tokens, &orphan_game_tokens);
        let tournaments = Self::collect_tournaments(&game_tokens, &orphan_game_tokens);

        // Games are created from game_tokens
        // while high and live map are generated using offer tokens
        let games: IndexVec<GameId, Game> = game_tokens
            .into_iter()
            .enumerate()
            .map(|(index, gt)| {
                let mut live_map = 0u64;
                let mut high_map = 0u64;
                offer_tokens
                    .iter()
                    .filter(|ot| ot.game_id == gt.id)
                    .for_each(|ot| {
                        let mapped_package_index = package_tokens
                            .iter()
                            .position(|pt| pt.id == ot.streaming_package_id)
                            .unwrap();
                        live_map.set_bit(mapped_package_index as u32, ot.live);
                        high_map.set_bit(mapped_package_index as u32, ot.highlights);
                    });

                Game {
                    id: GameId::new(index),
                    attributes: GameAttributes::from_token(&gt, &teams, &tournaments),
                    live_map,
                    high_map,
                }
            })
            .collect();

        let orphan_games: IndexVec<OrphanGameId, OrphanGame> = orphan_game_tokens
            .into_iter()
            .enumerate()
            .map(|(index, gt)| OrphanGame {
                id: OrphanGameId::new(index),
                attributes: GameAttributes::from_token(&gt, &teams, &tournaments),
            })
            .collect();

        let packages: IndexVec<PackageId, Package> = package_tokens
            .into_iter()
            .enumerate()
            .map(|(index, pt)| Package {
                id: PackageId::new(index),
                name: pt.name,
                monthly_price_cents: pt.monthly_price_cents,
                monthly_price_yearly_subscription_cents: pt
                    .monthly_price_yearly_subscription_in_cents,
            })
            .collect();

        Data {
            teams,
            tournaments,
            games,
            orphan_games,
            packages,
        }
    }

    #[throws(anyhow::Error)]
    pub fn load_from_bin(bytes: &[u8]) -> Self {
        bincode::deserialize(bytes)?
    }

    fn collect_tournaments(
        game_tokens: &[GameToken],
        orphan_game_tokens: &[GameToken],
    ) -> IndexVec<TournamentId, Tournament> {
        game_tokens
            .iter()
            .chain(orphan_game_tokens.iter())
            .map(|g| g.tournament_name.clone())
            .unique()
            .map(Tournament)
            .collect()
    }

    fn collect_teams(
        game_tokens: &[GameToken],
        orphan_game_tokens: &[GameToken],
    ) -> IndexVec<TeamId, Team> {
        let teams: IndexVec<TeamId, Team> = {
            let mut team_map: HashMap<Team, u16> = HashMap::new();

            game_tokens
                .iter()
                .chain(orphan_game_tokens.iter())
                .flat_map(|game| [Team(game.team_home.clone()), Team(game.team_away.clone())])
                .for_each(|team| match team_map.get_mut(&team) {
                    Some(count) => *count += 1,
                    None => {
                        if team_map.insert(team, 1).is_some() {
                            unreachable!()
                        }
                    }
                });

            team_map
                .into_iter()
                .sorted_by_key(|&(_, count)| std::cmp::Reverse(count))
                .map(|(team, _)| team)
                .collect()
        };
        teams
    }
}

time::serde::format_description!(
    my_format,
    PrimitiveDateTime,
    "[year]-[month]-[day] [hour]:[minute]:[second]"
);

#[derive(Debug, Deserialize, Clone)]
struct GameToken {
    id: u16,
    team_home: SmolStr,
    team_away: SmolStr,
    #[serde(with = "my_format")]
    starts_at: PrimitiveDateTime,
    tournament_name: SmolStr,
}
impl ParsableFromCSV for GameToken {}

#[derive(Debug, Deserialize)]
struct OfferToken {
    game_id: u16,
    streaming_package_id: u8,
    #[serde(deserialize_with = "deserialize_bool_from_01")]
    live: bool,
    #[serde(deserialize_with = "deserialize_bool_from_01")]
    highlights: bool,
}
impl ParsableFromCSV for OfferToken {}

#[derive(Debug, Deserialize)]
struct PackageToken {
    id: u8,
    name: SmolStr,
    monthly_price_cents: Option<u32>,
    monthly_price_yearly_subscription_in_cents: u32,
}

impl ParsableFromCSV for PackageToken {}

// TODO: Consider if it makes sense for this to return an iterator instead. For performance reasons. Benchmarks necessary.
trait ParsableFromCSV: DeserializeOwned {
    #[throws(anyhow::Error)]
    fn parse_items_from_csv(path: &Path) -> Vec<Self> {
        csv::Reader::from_path(path)?
            .deserialize::<Self>()
            .collect::<Result<Vec<Self>, _>>()?
    }
}

#[throws(D::Error)]
fn deserialize_bool_from_01<'de, D: Deserializer<'de>>(deserializer: D) -> bool {
    let ch = char::deserialize(deserializer)?;
    match ch {
        '0' => false,
        '1' => true,
        x => panic!("Expected 0 or 1, got {x} instead."),
    }
}
