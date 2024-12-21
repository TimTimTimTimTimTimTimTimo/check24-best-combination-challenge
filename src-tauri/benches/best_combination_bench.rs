use std::hint::black_box;

use best_combination_data::Data;
use best_combination_lib::{
    best_combination_all, best_combination_multi_1, best_combination_multi_2,
    best_combination_single, load_data,
};
use tango_bench::*;

fn best_combination_benchmarks() -> impl IntoBenchmarks {
    let data: &'static Data = Box::leak(Box::new(load_data()));

    [
        benchmark_fn("benchmark_single", |b| {
            b.iter(|| best_combination_single(black_box(data)))
        }),
        benchmark_fn("benchmark_multi_1", |b| {
            b.iter(|| best_combination_multi_1(black_box(data)))
        }),
        benchmark_fn("benchmark_all", |b| {
            b.iter(|| best_combination_all(black_box(data)))
        }),
        benchmark_fn("benchmark_multi_2", |b| {
            b.iter(|| best_combination_multi_2(black_box(data)))
        }),
    ]
}

tango_benchmarks!(best_combination_benchmarks());
tango_main!();
