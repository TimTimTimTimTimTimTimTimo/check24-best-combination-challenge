use std::hint::black_box;

use best_combination_lib::{best_combination_single, load_data};
use betting_game_data::Data;
use tango_bench::{benchmark_fn, tango_benchmarks, tango_main, IntoBenchmarks};

fn best_combination_benchmarks() -> impl IntoBenchmarks {
    let data: &'static Data = Box::leak(Box::new(load_data()));

    return [benchmark_fn("benchmark_single", |b| {
        b.iter(|| best_combination_single(black_box(data)))
    })];
}

tango_benchmarks!(best_combination_benchmarks());
tango_main!();
