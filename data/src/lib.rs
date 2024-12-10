use std::{collections::HashMap, fs::File, io::BufWriter, path::Path};

use chrono::NaiveDateTime;
use derive_more::derive::Display;
use fehler::throws;
use index_vec::IndexVec;
use itertools::Itertools;
use serde::{de::DeserializeOwned, Deserialize, Deserializer, Serialize};
use smol_str::SmolStr;

#[derive(Debug, Deserialize)]
struct GameToken {
    id: u16,
    team_home: SmolStr,
    team_away: SmolStr,
    #[serde(deserialize_with = "deserialize_naive_datetime")]
    starts_at: NaiveDateTime,
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

// TODO: find nicer place for this function, recover the error don't panic
#[throws(D::Error)]
fn deserialize_naive_datetime<'de, D: Deserializer<'de>>(deserializer: D) -> NaiveDateTime {
    let str = String::deserialize(deserializer)?;
    NaiveDateTime::parse_from_str(&str, "%Y-%m-%d %H:%M:%S%.f").expect("Datetime parsing failed.")
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

#[derive(Serialize, Deserialize, Clone)]
pub struct Game {
    pub id: GameId,
    pub team_home_id: TeamId,
    pub team_away_id: TeamId,
    pub starts_at: NaiveDateTime,
    pub tournament_id: TournamentId,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Offer {
    pub id: OfferId,
    pub game_id: GameId,
    pub package_id: PackageId,
    pub live: bool,
    pub highlights: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Package {
    pub id: PackageId,
    pub name: SmolStr,
    pub monthly_price_cents: Option<u32>,
    pub monthly_price_yearly_subscription_in_cents: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Display, PartialEq, Eq, Hash)]
pub struct Team(SmolStr);

#[derive(Debug, Serialize, Deserialize, Clone, Display, PartialEq, Eq)]
pub struct Tournament(SmolStr);

index_vec::define_index_type! {
    pub struct GameId = u16;
}
index_vec::define_index_type! {
    pub struct OfferId = u16;
}
index_vec::define_index_type! {
    pub struct PackageId = u8;
}
index_vec::define_index_type! {
    pub struct TeamId = u16;
}
index_vec::define_index_type! {
    pub struct TournamentId = u16;
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub teams: IndexVec<TeamId, Team>,
    pub tournaments: IndexVec<TournamentId, Tournament>,
    pub games: IndexVec<GameId, Game>,
    pub offers: IndexVec<OfferId, Offer>,
    pub packages: IndexVec<PackageId, Package>,
}

impl Data {
    #[throws(anyhow::Error)]
    pub fn load_from_csv(games_path: &Path, offers_path: &Path, packages_path: &Path) -> Data {
        let game_tokens = GameToken::parse_items_from_csv(games_path)?;
        let offer_tokens = OfferToken::parse_items_from_csv(offers_path)?;
        let package_tokens = PackageToken::parse_items_from_csv(packages_path)?;

        let offers = offer_tokens
            .iter()
            .enumerate()
            .map(|(i, ot)| {
                let game_id = ot.game_id;
                let package_id = ot.streaming_package_id;

                let game_index = game_tokens
                    .iter()
                    .position(|gt| gt.id == game_id)
                    .ok_or(anyhow::anyhow!("Game ID {game_id} not found"))?;
                let package_index = package_tokens
                    .iter()
                    .position(|pt| pt.id == package_id)
                    .ok_or(anyhow::anyhow!("Package ID {package_id} not found"))?;

                Ok(Offer {
                    id: OfferId::new(i),
                    game_id: GameId::new(game_index),
                    package_id: PackageId::new(package_index),
                    live: ot.live,
                    highlights: ot.highlights,
                })
            })
            .collect::<Result<IndexVec<OfferId, Offer>, anyhow::Error>>()?;

        // teams are sorted by how many games they have
        let teams: IndexVec<TeamId, Team> = {
            let mut team_map: HashMap<Team, u16> = HashMap::new();

            game_tokens
                .iter()
                .flat_map(|game| [Team(game.team_home.clone()), Team(game.team_away.clone())])
                .for_each(|team| match team_map.get_mut(&team) {
                    Some(count) => *count += 1,
                    None => {
                        if let Some(_) = team_map.insert(team, 1) {
                            unreachable!()
                        }
                    }
                });

            team_map
                .into_iter()
                .sorted_by_key(|&(_, count)| count)
                .rev()
                .map(|(team, _)| team)
                .collect()
        };

        let tournaments: IndexVec<TournamentId, Tournament> = game_tokens
            .iter()
            .map(|g| g.tournament_name.clone())
            .unique()
            .map(|s| Tournament(s))
            .collect();

        let games = game_tokens
            .into_iter()
            .enumerate()
            .map(|(index, gt)| Game {
                id: GameId::new(index),
                team_home_id: TeamId::new(
                    teams
                        .iter()
                        .position(|t| *t.0 == gt.team_home)
                        .expect("team was not found."),
                ),
                team_away_id: TeamId::new(
                    teams
                        .iter()
                        .position(|t| *t.0 == gt.team_away)
                        .expect("team was not found."),
                ),
                starts_at: gt.starts_at,
                tournament_id: TournamentId::new(
                    tournaments
                        .iter()
                        .position(|t| *t.0 == gt.tournament_name)
                        .expect("tournament was not found."),
                ),
            })
            .collect();

        let packages = package_tokens
            .into_iter()
            .enumerate()
            .map(|(index, pt)| Package {
                id: PackageId::new(index),
                name: pt.name,
                monthly_price_cents: pt.monthly_price_cents,
                monthly_price_yearly_subscription_in_cents: pt
                    .monthly_price_yearly_subscription_in_cents,
            })
            .collect();

        Data {
            teams,
            tournaments,
            games,
            offers,
            packages,
        }
    }

    #[throws(anyhow::Error)]
    pub fn write_to_bin_file(&self, path: &Path) {
        let file = File::create(path)?;
        bincode::serialize_into(file, &self)?;
    }

    #[throws(anyhow::Error)]
    pub fn load_from_bin(bytes: &[u8]) -> Self {
        bincode::deserialize(bytes)?
    }

    #[throws(anyhow::Error)]
    pub fn write_ts_types(&self, path: &Path) {
        use std::io::Write; // Ensure the Write trait is in scope

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
              monthly_price_yearly_subscription_in_cents: number;
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
    }
}
