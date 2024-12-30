use best_combination_core::*;
use data::Data;
use fehler::throws;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage::<Data>(load_data())
        .invoke_handler(tauri::generate_handler![fetch_combinations_handler])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[throws(())]
#[tauri::command(rename_all = "snake_case")]
async fn fetch_combinations_handler(
    query: CombinationsQuery,
    state: tauri::State<'_, Data>,
) -> CombinationsResult {
    best_combination_core::fetch_combinations(state.inner(), query)
}
