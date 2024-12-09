use betting_game_data::{Data, Game, Offer, Package};
use fehler::throws;
use itertools::Itertools;
use serde::Serialize;

#[derive(Serialize, Default)]
pub struct Combination {
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
    let filtered_offers: Vec<Offer> = data
        .offers
        .iter()
        .filter(|o| game_ids.contains(&o.game_id))
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

    fn find_best_package(packages: &[Package], offers: &[Offer]) -> Package {
        debug_assert!(!packages.is_empty());
        packages
            .iter()
            .min_by(|p1, p2| {
                let p1_coverage = offers.iter().filter(|o| o.package_id == p1.id).count();
                let p2_coverage = offers.iter().filter(|o| o.package_id == p2.id).count();

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
    team_ids: Vec<u16>,
    state: tauri::State<'_, Data>,
) -> FetchCombinationsResponse {
    let filtered_game_ids: Vec<u16> = state
        .games
        .iter()
        .filter(|game| {
            team_ids
                .iter()
                .any(|&t_id| t_id == game.team_away_id || t_id == game.team_home_id)
        })
        .map(|g| g.id)
        .collect();

    println!("{:?}", filtered_game_ids);

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
    tauri::Builder::default()
        .manage::<Data>(load_data())
        .invoke_handler(tauri::generate_handler![fetch_combinations])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub fn load_data() -> Data {
    let data_bin = include_bytes!("../betting_game.dat");
    let data = Data::load_from_bin(data_bin).unwrap();
    data
}

// Bayern München
pub fn best_combination_single(data: &Data) -> Combination {
    let game_ids: &[u16] = &[
        51, 68, 75, 78, 88, 102, 112, 120, 124, 138, 145, 150, 160, 170, 185, 192, 195, 211, 213,
        218, 224, 239, 250, 256, 260, 271, 283, 292, 301, 306, 319, 324, 336, 348, 355, 5301, 5316,
        5321, 5326, 5337, 5345, 5360, 5363, 5379, 5382, 5390, 5400, 5412, 5418, 5432, 5436, 5445,
        5455, 5463, 5470, 5479, 5488, 5497, 5507, 5521, 5525, 5537, 5544, 5553, 5562, 5569, 5580,
        5589, 7349, 7885, 8434, 8460, 8480, 8497, 8508, 8527, 8554, 8562, 8839,
    ];

    find_best_combination(game_ids, data)
}
