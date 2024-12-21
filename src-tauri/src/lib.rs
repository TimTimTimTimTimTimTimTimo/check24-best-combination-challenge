use std::ops::{BitAnd, BitOr};

use best_combination_data::{CoverageMap, Data, Game, GameId, Package, PackageId, Team, TeamId};
use fehler::throws;
use itertools::Itertools;
use serde::Serialize;
use smol_str::ToSmolStr;
use wide::u16x16;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage::<Data>(load_data())
        .invoke_handler(tauri::generate_handler![fetch_combinations_by_teams])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Serialize, Default)]
pub struct Combination {
    package_ids: Vec<PackageId>,
}

#[derive(Serialize, Default)]
pub struct CombinationProperties {
    live_coverage: u16,
    high_coverage: u16,
    total_coverage: u16,
    price: u32,
}

impl Combination {
    /// Calculate coverage data and price for a given combination.
    fn calculate_properties(&self, maps: &MapsFromGames, data: &Data) -> CombinationProperties {
        let MapsFromGames {
            live_maps,
            high_maps,
            total_maps,
        } = maps;

        let mut package_map: u64 = 0;
        for p_id in &self.package_ids {
            package_map.set_bit(p_id.index(), true);
        }

        let live_coverage = live_maps
            .iter()
            .fold(0, |acc, map| acc + (map.bitand(package_map) != 0) as u16);

        let high_coverage = high_maps
            .iter()
            .fold(0, |acc, map| acc + (map.bitand(package_map) != 0) as u16);

        let total_coverage = total_maps
            .iter()
            .fold(0, |acc, map| acc + (map.bitand(package_map) != 0) as u16);

        assert!(live_coverage <= total_coverage);
        assert!(high_coverage <= total_coverage);
        assert!(
            total_coverage as usize <= live_maps.len(),
            "total: {}, game_count: {}",
            total_coverage,
            live_maps.len()
        );

        let price: u32 = self
            .package_ids
            .iter()
            .map(|&p_id| data.packages[p_id].monthly_price_yearly_subscription_in_cents)
            .sum();

        CombinationProperties {
            live_coverage,
            high_coverage,
            total_coverage,
            price,
        }
    }
}

/// Collects all live and highlight maps. Also generates total maps for the given games.
fn collect_maps_from_games(games: &[&Game]) -> MapsFromGames {
    debug_assert!(games.iter().map(|g| g.id).all_unique());

    let mut live_maps: Vec<u64> = Vec::with_capacity(games.len());
    let mut high_maps: Vec<u64> = Vec::with_capacity(games.len());
    let mut total_maps: Vec<u64> = Vec::with_capacity(games.len());
    for game in games {
        live_maps.push(game.live_map);
        high_maps.push(game.high_map);
        total_maps.push(game.live_map.bitor(game.high_map));
    }

    assert!(live_maps.len() == high_maps.len() && high_maps.len() == total_maps.len());
    MapsFromGames {
        live_maps,
        high_maps,
        total_maps,
    }
}

struct MapsFromGames {
    live_maps: Vec<u64>,
    high_maps: Vec<u64>,
    total_maps: Vec<u64>,
}

/// Vertically sums up bits across the maps
fn calculate_coverages(maps: &[u64]) -> [u16; 64] {
    // Split across 4 vectors, as wide only support u16x16 vectors at max, but we need to have u16x64
    let mut vectors = [
        u16x16::splat(0),
        u16x16::splat(0),
        u16x16::splat(0),
        u16x16::splat(0),
    ];

    // Coverage is calculated in 16 element chunks.
    // For each chunk a chunk of the map is extracted/expanded into a vector.
    // The vectors are then summed up to represent coverage information for each package.
    let mut map_chunk = u16x16::splat(0);
    for map in maps {
        for (i, vector) in vectors.iter_mut().enumerate() {
            for j in 0..16 {
                map_chunk.as_array_mut()[j] = map.get_bit(i * 16 + j) as u16;
            }
            *vector += map_chunk
        }
    }

    // Safe, as their internal memory represenations are the same. This prevents unnecessary allocations.
    let result: [u16; 64] = unsafe { std::mem::transmute::<[u16x16; 4], [u16; 64]>(vectors) };
    debug_assert!(result.iter().max().unwrap().to_owned() as usize <= maps.len());
    result
}

#[derive(Serialize)]
pub struct FetchCombinationsResponse {
    game_count: u16,
    orphan_count: u16,
    best_combination: Combination,
    best_combination_properties: CombinationProperties,
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

/// Filters games and orphans based on the filter predicate and then passes the games maps
/// to find_best_combination_greedy or find_best_combination_exhaustive, based on their number.
fn fetch_combinations<F: Fn(&Game) -> bool>(data: &Data, filter: F) -> FetchCombinationsResponse {
    // TODO: this sucks, is ugly and probably wrong
    let orphan_count = data
        .orphan_games
        .iter()
        .filter(|orphan_game| {
            filter(&Game {
                id: GameId::new(orphan_game.id.into()),
                team_home_id: orphan_game.team_home_id,
                team_away_id: orphan_game.team_away_id,
                start_time: orphan_game.start_time,
                tournament_id: orphan_game.tournament_id,
                live_map: 0,
                high_map: 0,
            })
        })
        .count() as u16;

    let filtered_games: Vec<&Game> = data.games.iter().filter(|game| filter(game)).collect();

    let maps = collect_maps_from_games(&filtered_games);

    let best_combination = find_best_combination_greedy(&maps, &data.packages.raw);
    let best_combination_properties = best_combination.calculate_properties(&maps, data);

    FetchCombinationsResponse {
        game_count: filtered_games.len() as u16,
        orphan_count,
        best_combination,
        best_combination_properties,
    }
}

fn find_best_combination_greedy(maps: &MapsFromGames, packages: &[Package]) -> Combination {
    let mut package_prices: [u32; 64] = [0; 64];

    for p in packages {
        package_prices[p.id.index()] = p.monthly_price_yearly_subscription_in_cents
    }

    let mut package_ids = vec![];

    let mut current_maps = maps.total_maps.clone();

    for _ in 0..packages.len() {
        let coverages = calculate_coverages(&current_maps);
        let best_package_id: PackageId = coverages
            .iter()
            .enumerate()
            .max_by_key(|(i, coverage)| (*coverage, std::cmp::Reverse(package_prices[*i])))
            .unwrap()
            .0
            .into();

        package_ids.push(best_package_id);
        current_maps.retain(|map| !map.get_bit(best_package_id.index()));

        if current_maps.is_empty() {
            break;
        }
    }

    Combination { package_ids }
}

pub fn load_data() -> Data {
    let data_bin = include_bytes!("../betting_game.dat");
    Data::load_from_bin(data_bin).unwrap()
}
// This is just used for testing and benchmarks. Should probably be in another file/module?

// Bayern München
pub fn best_combination_single(data: &Data) -> FetchCombinationsResponse {
    let team_id = data
        .teams
        .position(|t| *t == Team("Bayern München".to_smolstr()))
        .unwrap();

    fetch_combinations(data, |game| {
        game.team_away_id == team_id || game.team_home_id == team_id
    })
}

// Hatayspor, Deutschland, Bayern München and Real Madrid
pub fn best_combination_multi_1(data: &Data) -> FetchCombinationsResponse {
    // to make test more accurate, precalc team ids
    let test_teams = ["Hatayspor", "Deutschland", "Bayern München", "Real Madrid"];
    let team_ids: Vec<TeamId> = data
        .teams
        .iter()
        .enumerate()
        .filter(|(_, t)| test_teams.contains(&t.0.as_str()))
        .map(|(index, _)| TeamId::new(index))
        .collect();

    fetch_combinations(data, |game| {
        team_ids.contains(&game.team_away_id) || team_ids.contains(&game.team_home_id)
    })
}

// ALLE
pub fn best_combination_all(data: &Data) -> FetchCombinationsResponse {
    fetch_combinations(data, |_| true)
}

// Oxford United, Los Angeles FC, AS Rom
pub fn best_combination_multi_2(data: &Data) -> FetchCombinationsResponse {
    // to make test more accurate, precalc team ids
    let test_teams = ["Oxford United", "Los Angeles FC", "AS Rom"];
    let team_ids: Vec<TeamId> = data
        .teams
        .iter()
        .enumerate()
        .filter(|(_, t)| test_teams.contains(&t.0.as_str()))
        .map(|(index, _)| TeamId::new(index))
        .collect();

    fetch_combinations(data, |game| {
        team_ids.contains(&game.team_away_id) || team_ids.contains(&game.team_home_id)
    })
}
