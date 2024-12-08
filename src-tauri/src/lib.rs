use betting_game_data::{Data, Game, Offer, Package};
use fehler::throws;
use itertools::Itertools;
use serde::Serialize;

#[derive(Serialize, Default)]
struct Combination {
    package_ids: Vec<u8>,
    live_coverage: u16,
    highlights_coverage: u16,
    total_coverage: u16,
    total_price: u32,
}

impl Combination {
    fn new(package_ids: &[u8], game_ids: &[u16], data: &Data) -> Self {
        let (live_coverage, highlights_coverage, total_coverage): (u16, u16, u16) = data
            .offers
            .iter()
            .filter(|offer| {
                package_ids
                    .iter()
                    .any(|&package_id| package_id == offer.package_id)
                    && game_ids.iter().any(|&g_id| g_id == offer.game_id)
            })
            .fold((0, 0, 0), |(acc_live, acc_high, acc_total), offer| {
                (
                    acc_live + if offer.live { 1 } else { 0 },
                    acc_high + if offer.highlights { 1 } else { 0 },
                    acc_total + 1,
                )
            });

        let total_price: u32 = package_ids
            .iter()
            .map(|&p_id| data.packages[p_id as usize].monthly_price_yearly_subscription_in_cents)
            .sum();

        Self {
            package_ids: package_ids.to_vec(),
            live_coverage,
            highlights_coverage,
            total_coverage,
            total_price,
        }
    }
}

fn find_best_combination(game_ids: &[u16], data: &Data) -> Combination {
    // let game_ids: &Vec<u16> = &data.games.iter().map(|g| g.id).collect();

    let filtered_offers: Vec<Offer> = data
        .offers
        .iter()
        .filter(|o| game_ids.contains(&o.game_id))
        // .filter(|o| o.live)
        .cloned()
        .collect();

    let mut package_ids = vec![];

    let mut remaining_packages: Vec<Package> = data.packages.to_owned();
    // removes any games which do not have offers at all TODO: better way? data is useful
    let mut remaining_game_ids: Vec<u16> =
        filtered_offers.iter().map(|o| o.game_id).unique().collect();

    let mut remaining_offers: Vec<Offer> = filtered_offers.to_owned();
    // TODO: this is kind off dangerous? Not really though
    while !remaining_game_ids.is_empty() && !remaining_packages.is_empty() {
        let best_pack = find_best_package(&remaining_packages, &remaining_offers);
        let covered_game_ids: Vec<u16> = remaining_offers
            .iter()
            .filter(|o| o.package_id == best_pack.id)
            .map(|o| o.game_id)
            .collect();

        remaining_game_ids.retain(|g| !covered_game_ids.contains(g));
        remaining_offers.retain(|o| remaining_game_ids.contains(&o.game_id));
        remaining_packages.retain(|p| p.id != best_pack.id);

        package_ids.push(best_pack.id);
    }

    return Combination::new(&package_ids, game_ids, data);

    fn find_best_package(packages: &[Package], filtered_offers: &[Offer]) -> Package {
        debug_assert!(!packages.is_empty());
        packages
            .iter()
            .min_by(|p1, p2| {
                let p1_coverage = filtered_offers
                    .iter()
                    .filter(|o| o.package_id == p1.id)
                    .count();
                let p2_coverage = filtered_offers
                    .iter()
                    .filter(|o| o.package_id == p2.id)
                    .count();

                p2_coverage.cmp(&p1_coverage).then(
                    p1.monthly_price_yearly_subscription_in_cents
                        .cmp(&p2.monthly_price_yearly_subscription_in_cents),
                )
            })
            .unwrap()
            .to_owned()
    }
}

#[derive(Serialize)]
struct FetchCombinationsResponse {
    game_count: u16,
    best_combination: Combination,
    single_combinations: Vec<Combination>,
}

#[throws(())]
#[tauri::command(rename_all = "snake_case")]
async fn fetch_combinations(
    teams: Vec<String>,
    state: tauri::State<'_, Data>,
) -> FetchCombinationsResponse {
    let filtered_game_ids: Vec<u16> = state
        .games
        .iter()
        .filter(|game| {
            teams
                .iter()
                .any(|team| team == game.team_away || team == game.team_home)
        })
        .map(|g| g.id)
        .collect();

    let single_combis: Vec<Combination> = state
        .packages
        .iter()
        .map(|p| Combination::new(&[p.id], &filtered_game_ids, state.inner()))
        .collect();

    FetchCombinationsResponse {
        game_count: filtered_game_ids.len() as u16,
        best_combination: find_best_combination(&filtered_game_ids, state.inner()),
        single_combinations: single_combis,
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let data_bin = include_bytes!("../betting_game.dat");

    tauri::Builder::default()
        .manage::<Data>(Data::load_from_bin(data_bin).unwrap())
        .invoke_handler(tauri::generate_handler![fetch_combinations])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
