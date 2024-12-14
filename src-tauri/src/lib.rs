use best_combination_data::{Data, GameId, Offer, Package, PackageId, TeamId};
use fehler::throws;
use fixedbitset::FixedBitSet;
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
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
                package_ids.contains(&offer.package_id) && game_ids.contains(&offer.game_id)
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

    let packages: Vec<Package> = filtered_offers
        .iter()
        .map(|o| o.package_id)
        .collect::<FxHashSet<_>>()
        .into_iter()
        .map(|p_id| &data.packages[p_id])
        .cloned()
        .collect();

    let best_package_ids = find_best_packages_greedy(game_ids.to_vec(), filtered_offers, packages);

    return Combination::new(&best_package_ids, &game_ids, data);
}

fn find_best_packages_greedy(
    game_ids: Vec<GameId>,
    offers: Vec<Offer>,
    packages: Vec<Package>,
) -> Vec<PackageId> {
    let game_id_to_index: FxHashMap<_, _> = game_ids
        .iter()
        .enumerate()
        .map(|(i, id)| (*id, i))
        .collect();

    // Create a lookup map for package offers
    let mut offer_by_package: FxHashMap<PackageId, Vec<usize>> = FxHashMap::default();
    for offer in &offers {
        if let Some(&game_index) = game_id_to_index.get(&offer.game_id) {
            offer_by_package
                .entry(offer.package_id)
                .or_default()
                .push(game_index);
        }
    }

    // Create the coverage maps
    let package_coverages: Vec<FixedBitSet> = packages
        .iter()
        .map(|p| {
            let mut package_coverage_map = FixedBitSet::with_capacity(game_ids.len());
            if let Some(indices) = offer_by_package.get(&p.id) {
                for &index in indices {
                    package_coverage_map.set(index, true);
                }
            }
            package_coverage_map
        })
        .collect();

    let package_prices: Vec<u32> = packages
        .iter()
        .map(|p| p.monthly_price_yearly_subscription_in_cents)
        .collect();

    let mut best_package_ids = vec![];
    let mut covered_games_map = FixedBitSet::with_capacity(game_ids.len());
    for _ in 0..packages.len() {
        if covered_games_map.is_full() {
            break;
        }
        // dbg!(&covered_games_map);
        // dbg!(&covered_games_map.count_ones(..));
        let (best_package_index, best_coverage_map) = package_coverages
            .iter()
            .enumerate()
            .max_by_key(|(index, cov_map)| {
                (
                    cov_map.difference_count(&covered_games_map),
                    std::cmp::Reverse(package_prices[*index]),
                )
            })
            .unwrap();

        best_package_ids.push(packages[best_package_index].id);
        covered_games_map.union_with(best_coverage_map);
    }

    best_package_ids
}

fn find_best_packages_exhaustive(
    game_ids: Vec<GameId>,
    offers: Vec<Offer>,
    packages: Vec<Package>,
) -> Vec<PackageId> {
    todo!()
}

#[derive(Serialize)]
struct FetchCombinationsResponse {
    game_count: u16,
    orphan_count: u16,
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

    let orphan_count = state
        .orphan_games
        .iter()
        .filter(|g| team_ids.contains(&g.team_away_id) || team_ids.contains(&g.team_home_id))
        .count() as u16;

    let single_combis: Vec<Combination> = state
        .packages
        .iter()
        .map(|p| Combination::new(&[p.id], &filtered_game_ids, state.inner()))
        .collect();

    let best_combination = find_best_combination(&filtered_game_ids, state.inner());
    let game_count = filtered_game_ids.len() as u16;
    assert!(
        best_combination.total_coverage <= game_count,
        "This is impossible, rethink your life! coverage: {}, count: {}",
        best_combination.total_coverage,
        game_count
    );

    FetchCombinationsResponse {
        game_count: filtered_game_ids.len() as u16,
        orphan_count,
        best_combination,
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
    // this is all prefiltered. should find a good way to get this at comptime
    let game_ids: Vec<GameId> = vec![
        51, 68, 75, 78, 88, 102, 112, 120, 124, 138, 145, 150, 160, 170, 185, 192, 195, 211, 213,
        218, 224, 239, 250, 256, 260, 271, 283, 292, 301, 306, 319, 324, 336, 348, 355, 3672, 3687,
        3692, 3697, 3708, 3716, 3731, 3734, 3750, 3753, 3761, 3771, 3783, 3789, 3803, 3807, 3816,
        3826, 3834, 3841, 3850, 3859, 3868, 3878, 3892, 3896, 3908, 3915, 3924, 3933, 3940, 3951,
        3960, 5405, 5466, 5505, 5531, 5551, 5568, 5579, 5619, 5627, 5652,
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
        348, 355, 929, 940, 951, 964, 975, 986, 996, 998, 1008, 1019, 1031, 1044, 1054, 1061, 1074,
        1080, 1089, 1101, 1107, 1125, 1135, 1143, 1152, 1160, 1174, 1178, 1196, 1204, 1216, 1218,
        1231, 1240, 1252, 1263, 1268, 1284, 1293, 1299, 1738, 1747, 1761, 1772, 1782, 1793, 1799,
        1816, 1825, 1828, 1840, 1853, 1858, 1868, 1884, 1889, 1907, 1916, 1921, 1932, 1938, 1948,
        1958, 1969, 1978, 1995, 2000, 2011, 2021, 2033, 2046, 2049, 2064, 2074, 2081, 2094, 2104,
        2110, 2127, 2623, 2626, 2628, 3672, 3687, 3692, 3697, 3708, 3716, 3731, 3734, 3750, 3753,
        3761, 3771, 3783, 3789, 3803, 3807, 3816, 3826, 3834, 3841, 3850, 3859, 3868, 3878, 3892,
        3896, 3908, 3915, 3924, 3933, 3940, 3951, 3960, 5052, 5058, 5072, 5081, 5089, 5101, 5112,
        5120, 5131, 5141, 5152, 5161, 5168, 5182, 5190, 5197, 5202, 5213, 5223, 5229, 5243, 5252,
        5260, 5272, 5283, 5291, 5302, 5312, 5323, 5332, 5339, 5353, 5361, 5368, 5373, 5384, 5405,
        5466, 5501, 5505, 5506, 5531, 5534, 5545, 5551, 5563, 5568, 5579, 5590, 5594, 5619, 5621,
        5627, 5638, 5652,
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
        793, 834, 856, 1311, 1319, 1328, 1344, 1356, 1366, 1373, 1385, 1390, 1403, 1413, 1425,
        1433, 1444, 1454, 1464, 1476, 1486, 1496, 1504, 1508, 1523, 1533, 1536, 1553, 1564, 1570,
        1586, 1596, 1606, 1610, 1624, 1636, 1645, 1655, 1665, 1675, 1684, 1723, 1726, 2130, 2144,
        2166, 2180, 2195, 2199, 2218, 2226, 2250, 2267, 2273, 2294, 2305, 2323, 2349, 2361, 2374,
        2388, 2406, 2419, 2435, 2444, 2461, 2474, 2491, 2493, 2516, 2537, 2549, 2558, 2567, 2590,
        2602, 2619, 3968, 3984, 3991, 4001, 4015, 4024, 4034, 4045, 4049, 4065, 4069, 4085, 4093,
        4104, 4115, 4119, 4134, 4143, 4154, 4157, 4176, 4185, 4195, 4206, 4215, 4224, 4235, 4239,
        4253, 4264, 4275, 4283, 4295, 4301, 4315, 4317, 4336, 4344, 5498,
    ]
    .into_iter()
    .map(GameId::from)
    .collect();

    find_best_combination(&game_ids, data)
}
