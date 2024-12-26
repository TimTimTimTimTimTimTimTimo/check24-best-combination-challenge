use algo::*;
use arrayvec::ArrayVec;
use data::*;
use num_traits::{PrimInt, Unsigned};
use serde::Serialize;
use smol_str::ToSmolStr;

mod algo;
pub mod data;

#[derive(Serialize, Default)]
pub struct Combination {
    pub package_ids: Vec<PackageId>,
}

#[derive(Serialize, Default)]
pub struct CombinationProperties {
    pub live_coverage: u16,
    pub high_coverage: u16,
    pub total_coverage: u16,
    pub price: u32,
}

impl Combination {
    /// Calculate coverage information and price for a given combination.
    fn calculate_properties(&self, maps: &MapsFromGames, data: &Data) -> CombinationProperties {
        let MapsFromGames {
            live_maps,
            high_maps,
            total_maps,
        } = maps;

        let mut package_map: u64 = 0;
        for p_id in &self.package_ids {
            package_map.set_bit(p_id.index() as u32, true);
        }

        let live_coverage = live_maps
            .iter()
            .fold(0, |acc, map| acc + ((map & package_map) != 0) as u16);

        let high_coverage = high_maps
            .iter()
            .fold(0, |acc, map| acc + ((map & package_map) != 0) as u16);

        let total_coverage = total_maps
            .iter()
            .fold(0, |acc, map| acc + ((map & package_map) != 0) as u16);

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
            .map(|&p_id| data.packages[p_id].monthly_price_yearly_subscription_cents)
            .sum();

        CombinationProperties {
            live_coverage,
            high_coverage,
            total_coverage,
            price,
        }
    }
}

#[derive(Serialize)]
pub struct FetchCombinationsResponse {
    game_count: u16,
    orphan_count: u16,
    best_combination: Combination,
    best_combination_properties: CombinationProperties,
}

/// Filters games based on the predicate and returns the optimal combination and its properties
pub fn fetch_combinations<F: Fn(&Game) -> bool>(
    data: &Data,
    filter: F,
) -> FetchCombinationsResponse {
    // TODO: this sucks, is ugly and probably wrong
    let orphan_count = data
        .orphan_games
        .iter()
        .filter(|orphan_game| {
            let game: Game = (*orphan_game).clone().into();
            filter(&game)
        })
        .count() as u16;

    let filtered_games: Vec<&Game> = data.games.iter().filter(|game| filter(game)).collect();
    let maps = collect_maps_from_games(&filtered_games);

    let best_combination = find_best_combination(&maps.total_maps, &data.packages);
    let best_combination_properties = best_combination.calculate_properties(&maps, data);

    FetchCombinationsResponse {
        game_count: filtered_games.len() as u16,
        orphan_count,
        best_combination,
        best_combination_properties,
    }
}

/// !!! BITS must equal the types size in bits !!!
/// Until Rust stabilizes const expressions this is the best I can do :(
// TODO: this might be sketchy. Testing would be great. If only time...
trait Bitmap<const BITS: usize>: Unsigned + PrimInt {
    #[inline]
    fn set_bit(&mut self, index: u32, value: bool) {
        debug_assert!(index < BITS as u32);
        *self = *self & !(Self::one() << index as usize)
            | (Self::from(value as u8).unwrap() << index as usize);
    }

    // #[inline]
    // fn set_bits(&mut self, indices: &[u32], value: bool) {
    //     let mask = indices.iter().fold(Self::zero(), |acc, &index| {
    //         debug_assert!(index < BITS as u32);
    //         acc | (Self::one() << index as usize)
    //     });
    //     *self = (*self & !mask) | (if value { mask } else { Self::zero() });
    // }

    #[inline]
    fn get_bit(&self, index: u32) -> bool {
        assert!(index < BITS as u32);
        !(*self & (Self::one() << index as usize)).is_zero()
    }

    // #[inline]
    // fn get_bits(&self) -> ArrayVec<u32, BITS> {
    //     // Got this from: https://www.reddit.com/r/rust/comments/r91ok5/comment/hn9ahxi/
    //     let mut x = *self;
    //     let mut result = ArrayVec::new();
    //     while x != Self::zero() {
    //         let index = x.trailing_zeros();
    //         result.push(index);
    //         x = x ^ (Self::one() << index as usize);
    //     }

    //     result
    // }
}

impl Bitmap<64> for u64 {}

pub fn load_data() -> Data {
    let data_bin = include_bytes!("../data/best_combination.dat");
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
