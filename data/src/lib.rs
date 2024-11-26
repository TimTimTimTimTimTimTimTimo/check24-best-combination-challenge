use std::{collections::HashSet, fs::File, io::BufWriter, path::Path};

use chrono::NaiveDateTime;
use fehler::throws;
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
    pub id: u16,
    pub team_home: SmolStr,
    pub team_away: SmolStr,
    pub starts_at: NaiveDateTime,
    pub tournament: SmolStr,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Offer {
    pub game_id: u16,
    pub package_id: u8,
    pub live: bool,
    pub highlights: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Package {
    pub id: u8,
    pub name: SmolStr,
    pub monthly_price_cents: Option<u32>,
    pub monthly_price_yearly_subscription_in_cents: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub games: Vec<Game>,
    pub offers: Vec<Offer>,
    pub packages: Vec<Package>,
}

impl Data {
    #[throws(anyhow::Error)]
    pub fn load_from_csv(games_path: &Path, offers_path: &Path, packages_path: &Path) -> Data {
        let game_tokens = GameToken::parse_items_from_csv(games_path)?;
        let offer_tokens = OfferToken::parse_items_from_csv(offers_path)?;
        let package_tokens = PackageToken::parse_items_from_csv(packages_path)?;

        let offers = offer_tokens
            .iter()
            .map(|ot| {
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
                    game_id: game_index.try_into()?,
                    package_id: package_index.try_into()?,
                    live: ot.live,
                    highlights: ot.highlights,
                })
            })
            .collect::<Result<Vec<Offer>, anyhow::Error>>()?;

        let games = game_tokens
            .into_iter()
            .enumerate()
            .map(|(index, gt)| Game {
                id: index as u16,
                team_home: gt.team_home,
                team_away: gt.team_away,
                starts_at: gt.starts_at,
                tournament: gt.tournament_name,
            })
            .collect();
        let packages = package_tokens
            .into_iter()
            .enumerate()
            .map(|(index, pt)| Package {
                id: index as u8,
                name: pt.name,
                monthly_price_cents: pt.monthly_price_cents,
                monthly_price_yearly_subscription_in_cents: pt
                    .monthly_price_yearly_subscription_in_cents,
            })
            .collect();

        Data {
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

        let unique_teams: HashSet<&str> = self
            .games
            .iter()
            .flat_map(|game| [game.team_home.as_str(), game.team_away.as_str()])
            .collect();

        writeln!(
            writer,
            "
            export const teams = [{}];
            export type Team = typeof teams[number];
            ",
            unique_teams
                .into_iter()
                .map(|team| format!("\"{team}\""))
                .join(",")
        )?;

        writeln!(writer)?;

        let unique_tournaments: HashSet<&str> = self
            .games
            .iter()
            .map(|game| game.tournament.as_str())
            .collect();

        writeln!(
            writer,
            "
            export const tournaments = [{}];
            export type Tournament = typeof tournaments[number];
            ",
            unique_tournaments
                .into_iter()
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
