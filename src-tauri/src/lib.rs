use std::{collections::HashMap, path::Path};

use betting_game_data::{Data, Game, Offer};
use fehler::throws;
use itertools::Itertools;
use serde::Serialize;

#[derive(Serialize)]
struct GamesAndOffers {
    games: Vec<Game>,
    offers_map: HashMap<u16, Vec<Offer>>,
}

#[throws(())]
#[tauri::command]
async fn find_games_and_offers_by_team(
    team: &str,
    state: tauri::State<'_, Data>,
) -> GamesAndOffers {
    let games: Vec<Game> = state
        .games
        .iter()
        .filter(|&game| game.team_away == team || game.team_home == team)
        .cloned()
        .collect();

    let offers_map: HashMap<u16, Vec<Offer>> = state
        .offers
        .iter()
        .filter(|offer| games.iter().any(|game| game.id == offer.game_id))
        .cloned()
        .into_group_map_by(|offer| offer.game_id);

    GamesAndOffers { games, offers_map }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage::<Data>(Data::load_from_bin(Path::new("./betting_game.dat")).unwrap())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![find_games_and_offers_by_team])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
