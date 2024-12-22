use std::ops::BitAnd;

use algo::*;
use data::*;
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

#[derive(Serialize)]
pub struct FetchCombinationsResponse {
    game_count: u16,
    orphan_count: u16,
    best_combination: Combination,
    best_combination_properties: CombinationProperties,
}

/// Filters games and orphans based on the filter predicate and then passes the games maps
/// to find_best_combination_greedy or find_best_combination_exhaustive, based on their number.
pub fn fetch_combinations<F: Fn(&Game) -> bool>(
    data: &Data,
    filter: F,
) -> FetchCombinationsResponse {
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
