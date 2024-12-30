use std::ops::RangeInclusive;

use algo::*;
use data::*;
use index_vec::{IndexSlice, IndexVec};
use itertools::Itertools;
use num_traits::{PrimInt, Unsigned};
use serde::{Deserialize, Serialize};
use smol_str::ToSmolStr;
use time::Date;

mod algo;
pub mod data;

#[derive(Serialize, Default)]
struct Combination {
    pub package_ids: Vec<PackageId>,
    pub coverages: Coverages,
    pub yearly_price_per_month_cents: u32,
}

impl Combination {
    /// Calculate coverage information and price for a given combination.
    fn calculate_coverages(
        package_ids: &[PackageId],
        games: &IndexSlice<GameId, [&Game]>,
    ) -> Coverages {
        let mut package_map: u64 = 0;
        for p_id in package_ids {
            package_map.set_bit(p_id.index() as u32, true);
        }

        let mut live_coverage = 0;
        let mut high_coverage = 0;
        let mut some_coverage = 0;
        let mut full_coverage = 0;

        for game in games {
            let has_live = (game.live_map & package_map) != 0;
            let has_high = (game.high_map & package_map) != 0;

            live_coverage += has_live as u16;
            high_coverage += has_high as u16;
            some_coverage += ((game.live_map | game.high_map) & package_map != 0) as u16;
            full_coverage += (has_live && has_high) as u16;
        }

        assert!(full_coverage <= live_coverage);
        assert!(full_coverage <= high_coverage);
        assert!(live_coverage <= some_coverage);
        assert!(high_coverage <= some_coverage);
        assert!(some_coverage as usize <= games.len());

        Coverages {
            high_coverage,
            live_coverage,
            some_coverage,
            full_coverage,
        }
    }

    /// Creates a Combination and calculates its properties.
    fn create(
        packages: &IndexSlice<PackageId, [&Package]>,
        games: &IndexSlice<GameId, [&Game]>,
    ) -> Combination {
        let package_ids = packages.iter().map(|p| p.id).collect_vec();
        let coverages = Self::calculate_coverages(&package_ids, games);

        Self {
            package_ids,
            coverages,
            yearly_price_per_month_cents: packages
                .iter()
                .map(|p| p.monthly_price_yearly_subscription_cents)
                .sum(),
        }
    }
}

#[derive(Deserialize, PartialEq)]
enum CoverType {
    /// Games can be watched via highlights.
    High,
    /// Games can be watched live.
    Live,
    /// Games can be watched.
    Some,
    /// Games can be watched live AND via highlights.
    Full,
}

#[derive(Serialize, Default)]
struct Coverages {
    pub high_coverage: u16,
    pub live_coverage: u16,
    pub some_coverage: u16,
    pub full_coverage: u16,
}

#[derive(Serialize)]
pub struct CombinationsResult {
    /// Number of games with streaming offers.
    game_count: u16,
    /// Number of games without streaming offers.
    orphan_count: u16,
    /// Cheapest combination which covers all games.
    cheapest_combination: Combination,
    /// Smallest combination which covers all games. None if cheapest_combination is also the smallest.
    smallest_combination: Option<Combination>,
    /// All packages as single combinations.
    single_combinations: Vec<Combination>,
}

#[derive(Deserialize)]
pub struct CombinationsQuery {
    /// Teams whose games need to be covered.
    team_ids: Vec<TeamId>,
    /// Tournaments the Teams participate in, whose games need to be covered.
    tournament_ids: Vec<TournamentId>,
    /// Timespan in which the games happen.
    timespan: RangeInclusive<Date>,
    /// Type of coverage required.
    cover_type: CoverType,
}

impl CombinationsQuery {
    fn contains_game(&self, attributes: &GameAttributes) -> bool {
        (self.team_ids.contains(&attributes.team_home_id)
            || self.team_ids.contains(&attributes.team_away_id))
            && self.tournament_ids.contains(&attributes.tournament_id)
            && self.timespan.contains(&attributes.date)
    }
}

/// Filters games and based on the query and returns the optimal combination and its properties
pub fn fetch_combinations(data: &Data, query: CombinationsQuery) -> CombinationsResult {
    let filtered_games: IndexVec<GameId, &Game> = data
        .games
        .iter()
        .filter(|g| query.contains_game(&g.attributes))
        .collect();

    let orphan_count = data
        .orphan_games
        .iter()
        .filter(|og| query.contains_game(&og.attributes))
        .count() as u16;

    let mut maps_count = filtered_games.len();
    if query.cover_type == CoverType::Full {
        maps_count *= 2
    }
    let mut maps = Vec::with_capacity(maps_count);
    for g in &filtered_games {
        match query.cover_type {
            CoverType::High => maps.push(g.high_map),
            CoverType::Live => maps.push(g.live_map),
            CoverType::Some => maps.push(g.high_map | g.live_map),
            CoverType::Full => {
                maps.push(g.high_map);
                maps.push(g.live_map);
            }
        }
    }

    let best_combinations = find_best_combinations(&maps, &data.packages);
    let cheapest_combination = {
        let cheapest_combination_packages: IndexVec<PackageId, &Package> = best_combinations
            .cheapest
            .iter()
            .map(|p_id| &data.packages[*p_id])
            .collect();
        Combination::create(&cheapest_combination_packages, &filtered_games)
    };

    let smallest_combination = best_combinations.smallest.map(|p_ids| {
        let smallest_combination_packages: IndexVec<PackageId, &Package> =
            p_ids.iter().map(|p_id| &data.packages[*p_id]).collect();
        Combination::create(&smallest_combination_packages, &filtered_games)
    });

    let single_combinations = data
        .packages
        .iter()
        .map(|p| Combination::create(IndexSlice::from_slice(&[p]), &filtered_games))
        .collect();

    CombinationsResult {
        game_count: filtered_games.len() as u16,
        orphan_count,
        cheapest_combination,
        smallest_combination,
        single_combinations,
    }
}

/// !!! BITS must equal the types size in bits !!!
/// Until Rust stabilizes const expressions this is the best I can do :(
trait Bitmap<const BITS: usize>: Unsigned + PrimInt {
    #[inline]
    fn set_bit(&mut self, index: u32, value: bool) {
        debug_assert!(index < BITS as u32);
        *self = *self & !(Self::one() << index as usize)
            | (Self::from(value as u8).unwrap() << index as usize);
    }

    #[inline]
    fn get_bit(&self, index: u32) -> bool {
        assert!(index < BITS as u32);
        !(*self & (Self::one() << index as usize)).is_zero()
    }
}

impl Bitmap<64> for u64 {}

pub fn load_data() -> Data {
    let data_bin = include_bytes!("../data/best_combination.dat");
    Data::load_from_bin(data_bin).unwrap()
}

// This is just used for testing and benchmarks.
/// Tests Bayern München.
pub fn best_combination_single(data: &Data) -> CombinationsResult {
    let team_id = data
        .teams
        .position(|t| *t == Team("Bayern München".to_smolstr()))
        .unwrap();

    let tournament_ids = (0..data.tournaments.len()).map(TournamentId::new).collect();
    fetch_combinations(
        data,
        CombinationsQuery {
            team_ids: vec![team_id],
            tournament_ids,
            timespan: Date::MIN..=Date::MAX,
            cover_type: CoverType::Some,
        },
    )
}

/// Tests Hatayspor, Deutschland, Bayern München and Real Madrid.
pub fn best_combination_multi_1(data: &Data) -> CombinationsResult {
    // to make test more accurate, precalc team ids
    let test_teams = ["Hatayspor", "Deutschland", "Bayern München", "Real Madrid"];
    let team_ids: Vec<TeamId> = data
        .teams
        .iter()
        .enumerate()
        .filter(|(_, t)| test_teams.contains(&t.0.as_str()))
        .map(|(index, _)| TeamId::new(index))
        .collect();

    let tournament_ids = (0..data.tournaments.len()).map(TournamentId::new).collect();

    fetch_combinations(
        data,
        CombinationsQuery {
            team_ids,
            tournament_ids,
            timespan: Date::MIN..=Date::MAX,
            cover_type: CoverType::Some,
        },
    )
}

/// Tests ALL Games.
pub fn best_combination_all(data: &Data) -> CombinationsResult {
    let team_ids = (0..data.teams.len()).map(TeamId::new).collect();
    let tournament_ids = (0..data.tournaments.len()).map(TournamentId::new).collect();
    fetch_combinations(
        data,
        CombinationsQuery {
            team_ids,
            tournament_ids,
            timespan: Date::MIN..=Date::MAX,
            cover_type: CoverType::Some,
        },
    )
}

/// Tests Oxford United, Los Angeles FC, AS Rom.
pub fn best_combination_multi_2(data: &Data) -> CombinationsResult {
    // to make test more accurate, precalc team ids
    let test_teams = ["Oxford United", "Los Angeles FC", "AS Rom"];
    let team_ids: Vec<TeamId> = data
        .teams
        .iter()
        .enumerate()
        .filter(|(_, t)| test_teams.contains(&t.0.as_str()))
        .map(|(index, _)| TeamId::new(index))
        .collect();

    let tournament_ids = (0..data.tournaments.len()).map(TournamentId::new).collect();
    fetch_combinations(
        data,
        CombinationsQuery {
            team_ids,
            tournament_ids,
            timespan: Date::MIN..=Date::MAX,
            cover_type: CoverType::Some,
        },
    )
}
