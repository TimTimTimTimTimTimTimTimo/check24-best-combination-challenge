use best_combination_core::*;
use data::{Data, TeamId};
use fehler::throws;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage::<Data>(load_data())
        .invoke_handler(tauri::generate_handler![fetch_combinations_by_teams])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[throws(())]
#[tauri::command(rename_all = "snake_case")]
async fn fetch_combinations_by_teams(
    team_ids: Vec<TeamId>,
    state: tauri::State<'_, Data>,
) -> FetchCombinationsResponse {
    fetch_combinations(state.inner(), |game| {
        team_ids.contains(&game.team_home_id) || team_ids.contains(&game.team_away_id)
    })
}
