use arrayvec::ArrayVec;
use fixedbitset::FixedBitSet;
use index_vec::IndexSlice;

use crate::{data::*, Bitmap};

#[derive(Debug)]
struct SearchFrame {
    uncovered_games_map: FixedBitSet,
    selected_pack_id: PackageId,
    /// First sorted by coverage ascending, then by price descending.
    /// Therefore the "best" element is always at the top of the stack.
    sorted_unused_pack_ids: ArrayVec<PackageId, 64>,
    current_price: u32,
}

pub struct BestCombinations {
    /// PackageIds which cover all games at the lowest price.
    pub cheapest: Vec<PackageId>,
    /// Smallest number of PackageIds which cover all games at the lowest price.
    pub smallest: Option<Vec<PackageId>>,
}

/// For the given maps, finds the best combinations to cover them all.
pub fn find_best_combinations(
    game_maps: &[u64],
    packs: &IndexSlice<PackageId, [Package]>,
) -> BestCombinations {
    // Collect for faster indexing later
    let package_prices: ArrayVec<u32, 64> = packs
        .iter()
        .map(|p| p.monthly_price_yearly_subscription_cents)
        .collect();

    // Used for passing into sort_packages_by_coverage, reused on each iteration to avoid new allocations
    let uncovered_game_masks = &mut game_maps.to_vec();
    uncovered_game_masks.shrink_to_fit();

    // Allocate only 64, as its the max number of packages
    let search_stack: &mut ArrayVec<SearchFrame, 64> = &mut ArrayVec::new();

    // Setup for iteration
    let first_frame = SearchFrame {
        uncovered_games_map: {
            let mut tmp = FixedBitSet::with_capacity(game_maps.len());
            tmp.set_range(.., true);
            tmp
        },
        // THIS IS A DUMMY VALUE, the first frame is always skipped when collecting package ids later
        selected_pack_id: PackageId::new(69),
        sorted_unused_pack_ids: {
            let mut unused_pack_ids: ArrayVec<PackageId, 64> = packs.iter().map(|p| p.id).collect();
            unused_pack_ids
                .sort_unstable_by_key(|p_id| std::cmp::Reverse(package_prices[p_id.index()]));
            sort_packages_by_coverage(uncovered_game_masks, &mut unused_pack_ids);
            unused_pack_ids
        },
        current_price: 0,
    };
    search_stack.push(first_frame);

    let cheapest_combination_pack_ids: &mut ArrayVec<PackageId, 64> = &mut ArrayVec::new();
    let mut cheapest_combination_price = u32::MAX;

    let smallest_combination_pack_ids: &mut ArrayVec<PackageId, 64> = &mut ArrayVec::new();
    smallest_combination_pack_ids.extend(packs.iter().map(|p| p.id));
    let mut smallest_combinations_price = u32::MAX;

    for _ in 0usize..(1 << packs.len()) {
        let current_frame = match search_stack.last_mut() {
            Some(frame) => frame,
            None => {
                break;
            }
        };

        // Check if there are still packages on this frame to check
        match current_frame.sorted_unused_pack_ids.pop() {
            None => {
                search_stack.pop();
            }
            Some(next_pack_id) => {
                // Check if adding the package makes the combination more expensive then the current best
                let next_price = current_frame.current_price + package_prices[next_pack_id.index()];
                if cheapest_combination_price <= next_price {
                    // It can't be better therefore check the next packages on this frame
                    continue;
                }

                // Calculate map with new package added
                let mut next_uncovered_games_map = current_frame.uncovered_games_map.clone();
                for game_index in current_frame.uncovered_games_map.ones() {
                    let game_mask = game_maps[game_index];
                    let is_covered = game_mask.get_bit(next_pack_id.index() as u32);
                    if is_covered {
                        next_uncovered_games_map.set(game_index, false);
                    }
                }

                // Check if the new package adds coverage at all
                if next_uncovered_games_map == current_frame.uncovered_games_map {
                    // The packages in the frame are always sorted by coverage,
                    // therefore the next packages will not add any coverage either, pop
                    search_stack.pop();
                    continue;
                }

                // Check if all games are covered
                if next_uncovered_games_map.is_clear() {
                    // Found new best solution
                    cheapest_combination_price = next_price;
                    cheapest_combination_pack_ids.clear();
                    cheapest_combination_pack_ids.extend(
                        search_stack
                            .iter()
                            .skip(1)
                            .map(|frame| frame.selected_pack_id),
                    );
                    cheapest_combination_pack_ids.push(next_pack_id);

                    // Check if smaller solution was found
                    if cheapest_combination_pack_ids.len() <= smallest_combination_pack_ids.len()
                        && cheapest_combination_price < smallest_combinations_price
                    {
                        smallest_combinations_price = cheapest_combination_price;
                        smallest_combination_pack_ids.clear();
                        smallest_combination_pack_ids
                            .try_extend_from_slice(cheapest_combination_pack_ids)
                            .unwrap();
                    }

                    continue;
                }

                // Further exploration of current combination is warranted
                uncovered_game_masks.clear();
                uncovered_game_masks.extend(
                    next_uncovered_games_map
                        .ones()
                        .map(|index| game_maps[index]),
                );

                let mut next_sorted_uncovered_pack_ids =
                    current_frame.sorted_unused_pack_ids.clone();

                // Nothing to sort if one
                if 1 < next_sorted_uncovered_pack_ids.len() {
                    next_sorted_uncovered_pack_ids.sort_unstable_by_key(|p_id| {
                        std::cmp::Reverse(package_prices[p_id.index()])
                    });
                    sort_packages_by_coverage(
                        uncovered_game_masks,
                        &mut next_sorted_uncovered_pack_ids,
                    );
                }

                // Setup for deeper exploration of the current combination
                search_stack.push(SearchFrame {
                    uncovered_games_map: next_uncovered_games_map,
                    selected_pack_id: next_pack_id,
                    sorted_unused_pack_ids: next_sorted_uncovered_pack_ids,
                    current_price: next_price,
                });
            }
        }
    }

    BestCombinations {
        cheapest: cheapest_combination_pack_ids.to_vec(),
        smallest: if cheapest_combination_pack_ids != smallest_combination_pack_ids {
            Some(smallest_combination_pack_ids.to_vec())
        } else {
            None
        },
    }
}

/// Calculates the pack_ids coverage using the game_masks, and then sorts the ids by coverage ascending.
fn sort_packages_by_coverage(game_maps: &[u64], pack_ids: &mut ArrayVec<PackageId, 64>) {
    // Sorting one or no elements makes no sense, hitting this indicates a logic error
    debug_assert!(2 <= pack_ids.len());
    debug_assert!(pack_ids.len() <= 64);

    let mut coverages: [u16; 64] = [0; 64];

    // Calculate coverages
    for mask in game_maps {
        for (i, pack_id) in pack_ids.iter().enumerate() {
            coverages[i] += ((mask >> pack_id.index()) & 1) as u16;
        }
    }

    // Scuffed insertion sort as using the std sort functions would have required allocating
    for current_pos in 1..pack_ids.len() {
        let mut insert_pos = current_pos;
        while insert_pos > 0 && coverages[insert_pos - 1] > coverages[insert_pos] {
            coverages.swap(insert_pos - 1, insert_pos);
            pack_ids.swap(insert_pos - 1, insert_pos);
            insert_pos -= 1;
        }
    }

    debug_assert!(coverages.len() >= pack_ids.len());
    debug_assert!(*coverages.iter().max().unwrap() <= game_maps.len() as u16);
}
