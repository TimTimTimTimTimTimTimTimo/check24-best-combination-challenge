use fehler::throws;
use parser::Data;
use parser::Game;
pub mod parser;

#[throws(())]
#[tauri::command]
async fn find_games_by_team(team: &str, state: tauri::State<'_, Data>) -> Vec<Game> {
    state
        .games
        .iter()
        .filter(|&game| game.team_away == team || game.team_home == team)
        .cloned()
        .collect()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage::<Data>(Data::init().unwrap())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![find_games_by_team])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
