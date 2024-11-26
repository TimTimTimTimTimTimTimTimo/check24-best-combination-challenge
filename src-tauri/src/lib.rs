use betting_game_data::{Data, Game, Offer};
use fehler::throws;
use serde::Serialize;

#[derive(Serialize)]
struct GamesAndCombinations {
    // Games sorted by date, Combinations by Coverage, then price TODO: encode this constraint into types
    games: Vec<Game>,
    combinations: Vec<Combination>,
}

impl GamesAndCombinations {
    fn new(team: &str, data: &Data) -> Self {
        let games = {
            let mut games: Vec<Game> = data
                .games
                .iter()
                .filter(|game| game.team_away == team || game.team_home == team)
                .cloned()
                .collect();
            games.sort_by_key(|game| game.starts_at);
            games
        };

        let combinations = {
            let mut combinations: Vec<Combination> = data
                .packages
                .iter()
                .map(|pack| Combination::new(&[pack.id], &games, data))
                .collect();

            combinations.sort_by_key(|combi| combi.total_price);
            // TODO: this can be optimized by simply reversing the cmp function.
            combinations.sort_by(|c1, c2| c2.offers.len().cmp(&c1.offers.len()));
            combinations
        };

        Self {
            games,
            combinations,
        }
    }
}

#[derive(Serialize)]
struct Combination {
    package_ids: Vec<u8>,
    offers: Vec<Offer>,
    total_price: u32,
}

impl Combination {
    fn new(packages: &[u8], games: &[Game], data: &Data) -> Self {
        let offers: Vec<Offer> = data
            .offers
            .iter()
            .filter(|offer| {
                packages
                    .iter()
                    .any(|&package_id| package_id == offer.package_id)
                    && games.iter().any(|game| game.id == offer.game_id)
            })
            .cloned()
            .collect();

        let total_price: u32 = packages
            .iter()
            .map(|&p_id| data.packages[p_id as usize].monthly_price_yearly_subscription_in_cents)
            .sum();

        Self {
            package_ids: packages.to_vec(),
            offers,
            total_price,
        }
    }
}

#[throws(())]
#[tauri::command]
async fn find_games_and_combinations_by_team(
    team: &str,
    state: tauri::State<'_, Data>,
) -> GamesAndCombinations {
    GamesAndCombinations::new(team, state.inner())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let data_bin = include_bytes!("../betting_game.dat");

    tauri::Builder::default()
        .manage::<Data>(Data::load_from_bin(data_bin).unwrap())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            find_games_and_combinations_by_team
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
