use anyhow::Result;
use chrono::NaiveDate;
use chrono::NaiveDateTime;
use fehler::throws;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde::Deserializer;
use smol_str::SmolStr;
use std::path::Path;

const DEFAULT_GAME_FILE_PATH: &str = "../data/bc_game.csv";
const DEFAULT_OFFER_FILE_PATH: &str = "../data/bc_streaming_offer.csv";
const DEFAULT_PACKAGE_FILE_PATH: &str = "../data/bc_streaming_package.csv";

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

#[derive(Debug, Default, Clone)]
pub struct Game {
    pub team_home: SmolStr,
    pub team_away: SmolStr,
    pub starts_at: NaiveDate,
    pub tournament_name: SmolStr,
}

impl From<GameToken> for Game {
    fn from(gt: GameToken) -> Self {
        Self {
            team_away: gt.team_away,
            team_home: gt.team_home,
            starts_at: gt.starts_at.date(),
            tournament_name: gt.tournament_name,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Offer {
    pub game_index: u16,
    pub package_index: u8,
    pub live: bool,
    pub highlights: bool,
}

#[derive(Debug, Clone, Default)]
pub struct Package {
    pub name: SmolStr,
    pub monthly_price_cents: Option<u32>,
    pub monthly_price_yearly_subscription_in_cents: u32,
}

impl From<PackageToken> for Package {
    fn from(pt: PackageToken) -> Self {
        Self {
            name: pt.name,
            monthly_price_cents: pt.monthly_price_cents,
            monthly_price_yearly_subscription_in_cents: pt
                .monthly_price_yearly_subscription_in_cents,
        }
    }
}

#[derive(Debug)]
pub struct Data {
    pub games: Vec<Game>,
    pub offers: Vec<Offer>,
    pub packages: Vec<Package>,
}

impl Data {
    #[throws(anyhow::Error)]
    pub fn init_with_paths(games_path: &Path, offers_path: &Path, packages_path: &Path) -> Data {
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
                    game_index: game_index.try_into()?,
                    package_index: package_index.try_into()?,
                    live: ot.live,
                    highlights: ot.highlights,
                })
            })
            .collect::<Result<Vec<Offer>, anyhow::Error>>()?;

        let games = game_tokens.into_iter().map(Game::from).collect();
        let packages = package_tokens.into_iter().map(Package::from).collect();

        Data {
            games,
            offers,
            packages,
        }
    }

    #[throws(anyhow::Error)]
    pub fn init() -> Self {
        let games_path = Path::new(DEFAULT_GAME_FILE_PATH);
        let offers_path = Path::new(DEFAULT_OFFER_FILE_PATH);
        let packages_path = Path::new(DEFAULT_PACKAGE_FILE_PATH);

        Self::init_with_paths(games_path, offers_path, packages_path)?
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    const GAME_TEST_FILE_PATH: &str = "..data/testing/bc_game_test.csv";
    const OFFER_TEST_FILE_PATH: &str = "../data/testing/bc_streaming_offer_test.csv";
    const PACKAGE_TEST_FILE_PATH: &str = "../data/testing/bc_streaming_package_test.csv";

    // TODO: Test negative path as well, and find nicer way to test
    #[test]
    #[throws(anyhow::Error)]
    fn parse_data() {
        let actual_data = Data::init_with_paths(
            Path::new(GAME_TEST_FILE_PATH),
            Path::new(OFFER_TEST_FILE_PATH),
            Path::new(PACKAGE_TEST_FILE_PATH),
        )?;

        assert_eq!(
            format!("{:?}", actual_data),
            format!(
                "{:?}",
                Data {
                    games: vec![
                        Game {
                            team_home: "1. FC Heidenheim 1846".into(),
                            team_away: "1. FC Köln".into(),
                            starts_at: NaiveDateTime::parse_from_str(
                                "2024-05-18 13:30:00",
                                "%Y-%m-%d %H:%M:%S"
                            )
                            .unwrap()
                            .date(),
                            tournament_name: "Bundesliga 23/24".into()
                        },
                        Game {
                            team_home: "VfL Bochum".into(),
                            team_away: "Fortuna Düsseldorf".into(),
                            starts_at: NaiveDateTime::parse_from_str(
                                "2024-05-23 18:30:00",
                                "%Y-%m-%d %H:%M:%S"
                            )
                            .unwrap()
                            .date(),
                            tournament_name: "Bundesliga 23/24".into()
                        },
                        Game {
                            team_home: "Fortuna Düsseldorf".into(),
                            team_away: "VfL Bochum".into(),
                            starts_at: NaiveDateTime::parse_from_str(
                                "2024-05-27 18:30:00",
                                "%Y-%m-%d %H:%M:%S"
                            )
                            .unwrap()
                            .date(),
                            tournament_name: "Bundesliga 23/24".into()
                        },
                        Game {
                            team_home: "SK Slovan Bratislava".into(),
                            team_away: "FC Struga Trim Lum".into(),
                            starts_at: NaiveDateTime::parse_from_str(
                                "2024-07-10 17:00:00",
                                "%Y-%m-%d %H:%M:%S"
                            )
                            .unwrap()
                            .date(),
                            tournament_name: "UEFA Champions League 24/25".into()
                        }
                    ],
                    offers: vec![
                        Offer {
                            game_index: 3,
                            package_index: 0,
                            live: true,
                            highlights: false
                        },
                        Offer {
                            game_index: 2,
                            package_index: 3,
                            live: false,
                            highlights: false
                        },
                        Offer {
                            game_index: 0,
                            package_index: 2,
                            live: true,
                            highlights: true
                        },
                        Offer {
                            game_index: 1,
                            package_index: 1,
                            live: false,
                            highlights: true
                        }
                    ],
                    packages: vec![
                        Package {
                            name: "DAZN - Unlimited".into(),
                            monthly_price_cents: Some(4499),
                            monthly_price_yearly_subscription_in_cents: 3499
                        },
                        Package {
                            name: "Sky - Bundesliga".into(),
                            monthly_price_cents: None,
                            monthly_price_yearly_subscription_in_cents: 3000
                        },
                        Package {
                            name: "Zattoo - SMART HD".into(),
                            monthly_price_cents: Some(649),
                            monthly_price_yearly_subscription_in_cents: 649
                        },
                        Package {
                            name: "Sky - Sport".into(),
                            monthly_price_cents: None,
                            monthly_price_yearly_subscription_in_cents: 2500
                        }
                    ]
                }
            )
        );
    }
}
