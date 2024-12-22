use std::ops::BitOr;

use itertools::Itertools;
use wide::u16x16;

use crate::{data::*, Combination};

/// Collects all live and highlight maps. Also generates total maps for the given games.
pub fn collect_maps_from_games(games: &[&Game]) -> MapsFromGames {
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

pub struct MapsFromGames {
    pub live_maps: Vec<u64>,
    pub high_maps: Vec<u64>,
    pub total_maps: Vec<u64>,
}

/// Takes in maps containing the coverage information for multiple games.
/// Returns an array containing the number of games covered by each of the individual packages.
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

pub fn find_best_combination_greedy(maps: &MapsFromGames, packages: &[Package]) -> Combination {
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
