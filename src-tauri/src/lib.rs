use betting_game_data::{Data, GameId, Offer, Package, PackageId, TeamId};
use fehler::throws;
use index_vec::IndexSlice;
use itertools::Itertools;
use serde::Serialize;

#[derive(Serialize, Default)]
pub struct Combination {
    package_ids: Vec<PackageId>,
    live_coverage: u16,
    highlights_coverage: u16,
    total_coverage: u16,
    total_price: u32,
}

impl Combination {
    fn new(package_ids: &[PackageId], game_ids: &[GameId], data: &Data) -> Self {
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
            .map(|&p_id| data.packages[p_id].monthly_price_yearly_subscription_in_cents)
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

fn find_best_combination(game_ids: &[GameId], data: &Data) -> Combination {
    let filtered_offers: Vec<Offer> = data
        .offers
        .iter()
        .filter(|o| game_ids.contains(&o.game_id))
        .cloned()
        .collect();

    let mut package_ids = vec![];

    let mut remaining_packages = data.packages.to_owned();
    // removes any games which do not have offers at all TODO: better way? data is useful
    let mut remaining_game_ids: Vec<_> =
        filtered_offers.iter().map(|o| o.game_id).unique().collect();

    let mut remaining_offers: Vec<Offer> = filtered_offers.to_owned();
    // TODO: this is kind off dangerous? Not really though
    while !remaining_game_ids.is_empty() && !remaining_packages.is_empty() {
        let best_pack = find_best_package(&remaining_packages, &remaining_offers);
        let covered_game_ids: Vec<_> = remaining_offers
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

    fn find_best_package(packages: &IndexSlice<PackageId, [Package]>, offers: &[Offer]) -> Package {
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
    team_ids: Vec<TeamId>,
    state: tauri::State<'_, Data>,
) -> FetchCombinationsResponse {
    let filtered_game_ids: Vec<GameId> = state
        .games
        .iter()
        .filter(|game| {
            team_ids
                .iter()
                .any(|&t_id| t_id == game.team_away_id || t_id == game.team_home_id)
        })
        .map(|g| g.id)
        .collect();

    // println!("{:?}", filtered_game_ids);

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

// This is just used for testing and benchmarks. Should probably be in another file/module?

// Bayern München
pub fn best_combination_single(data: &Data) -> Combination {
    let game_ids: Vec<GameId> = vec![
        51, 68, 75, 78, 88, 102, 112, 120, 124, 138, 145, 150, 160, 170, 185, 192, 195, 211, 213,
        218, 224, 239, 250, 256, 260, 271, 283, 292, 301, 306, 319, 324, 336, 348, 355, 5301, 5316,
        5321, 5326, 5337, 5345, 5360, 5363, 5379, 5382, 5390, 5400, 5412, 5418, 5432, 5436, 5445,
        5455, 5463, 5470, 5479, 5488, 5497, 5507, 5521, 5525, 5537, 5544, 5553, 5562, 5569, 5580,
        5589, 7349, 7885, 8434, 8460, 8480, 8497, 8508, 8527, 8554, 8562, 8839,
    ]
    .into_iter()
    .map(GameId::from)
    .collect();

    find_best_combination(&game_ids, data)
}

// Hatayspor, Deutschland, Bayern München and Real Madrid
pub fn best_combination_multi_1(data: &Data) -> Combination {
    let game_ids: Vec<GameId> = vec![
        0, 13, 24, 37, 44, 51, 68, 75, 78, 88, 102, 112, 120, 124, 138, 145, 150, 160, 170, 185,
        192, 195, 211, 213, 218, 224, 239, 250, 256, 260, 271, 283, 292, 301, 306, 319, 324, 336,
        348, 355, 1120, 1131, 1142, 1155, 1166, 1177, 1187, 1189, 1199, 1210, 1222, 1235, 1245,
        1252, 1265, 1271, 1280, 1292, 1298, 1316, 1326, 1334, 1343, 1351, 1365, 1369, 1387, 1395,
        1407, 1409, 1422, 1431, 1443, 1454, 1459, 1475, 1484, 1490, 1598, 1616, 2566, 2575, 2589,
        2600, 2610, 2621, 2627, 2644, 2653, 2656, 2668, 2681, 2686, 2696, 2712, 2717, 2735, 2744,
        2749, 2760, 2766, 2776, 2786, 2797, 2806, 2823, 2828, 2839, 2849, 2861, 2874, 2877, 2892,
        2902, 2909, 2922, 2932, 2938, 2955, 4217, 4220, 4222, 5301, 5316, 5321, 5326, 5337, 5345,
        5360, 5363, 5379, 5382, 5390, 5400, 5412, 5418, 5432, 5436, 5445, 5455, 5463, 5470, 5479,
        5488, 5497, 5507, 5521, 5525, 5537, 5544, 5553, 5562, 5569, 5580, 5589, 6987, 6993, 7007,
        7016, 7024, 7036, 7047, 7055, 7066, 7076, 7087, 7096, 7103, 7117, 7125, 7132, 7137, 7148,
        7158, 7164, 7178, 7187, 7195, 7207, 7218, 7226, 7237, 7247, 7258, 7267, 7274, 7288, 7296,
        7303, 7308, 7319, 7349, 7885, 8413, 8434, 8435, 8460, 8463, 8474, 8480, 8492, 8497, 8508,
        8519, 8523, 8527, 8554, 8556, 8562, 8573, 8839,
    ]
    .into_iter()
    .map(GameId::from)
    .collect();

    find_best_combination(&game_ids, data)
}

// ALLE
pub fn best_combination_all(data: &Data) -> Combination {
    let game_ids: Vec<GameId> = data.games.iter().map(|g| g.id).collect();

    find_best_combination(&game_ids, data)
}

// Oxford United, Los Angeles FC, AS Rom
pub fn best_combination_multi_2(data: &Data) -> Combination {
    let game_ids: Vec<GameId> = vec![
        891, 932, 954, 1055, 1630, 1638, 1647, 1663, 1675, 1685, 1692, 1704, 1709, 1722, 1732,
        1744, 1752, 1763, 1773, 1783, 1795, 1805, 1815, 1823, 1827, 1842, 1852, 1855, 1872, 1883,
        1889, 1905, 1915, 1925, 1929, 1943, 1955, 1964, 1974, 1984, 1994, 2003, 2042, 2045, 3724,
        3738, 3760, 3774, 3789, 3793, 3812, 3820, 3844, 3861, 3867, 3888, 3899, 3917, 3943, 3955,
        3968, 3982, 4000, 4013, 4029, 4038, 4055, 4068, 4085, 4087, 4110, 4131, 4143, 4152, 4161,
        4184, 4196, 4213, 5145, 5597, 5613, 5620, 5630, 5644, 5653, 5663, 5674, 5678, 5694, 5698,
        5714, 5722, 5733, 5744, 5748, 5763, 5772, 5783, 5786, 5805, 5814, 5824, 5835, 5844, 5853,
        5864, 5868, 5882, 5893, 5904, 5912, 5924, 5930, 5944, 5946, 5965, 5973, 8282, 8410, 8587,
        8607, 8613, 8638, 8664, 8666, 8685, 8702,
    ]
    .into_iter()
    .map(GameId::from)
    .collect();

    find_best_combination(&game_ids, data)
}
