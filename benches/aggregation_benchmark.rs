use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::Rng;

use project::aggregations::{
    adaptive_sum, adaptive_count, adaptive_avg, adaptive_min, adaptive_max,
    adaptive_distinct_count,
    normal_sum, parallel_sum, normal_count, parallel_count,
    normal_avg, parallel_avg, normal_min, parallel_min,
    normal_max, parallel_max, normal_distinct_count, parallel_distinct_count,
};

fn generate_random_data(size: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..size).map(|_| rng.gen_range(1..1000)).collect()
}

fn bench_aggregations(c: &mut Criterion) {
    let small_data = generate_random_data(10_000);
    let medium_data = generate_random_data(100_000);
    let large_data = generate_random_data(1_000_000);

    let mut group = c.benchmark_group("Aggregation Operations");

    // Sum benchmarks
    group.bench_function("Normal Sum (Small)", |b| b.iter(|| normal_sum(black_box(&small_data))));
    group.bench_function("Parallel Sum (Small)", |b| b.iter(|| parallel_sum(black_box(&small_data))));
    group.bench_function("Adaptive Sum (Small)", |b| b.iter(|| adaptive_sum(black_box(&small_data))));

    group.bench_function("Normal Sum (Medium)", |b| b.iter(|| normal_sum(black_box(&medium_data))));
    group.bench_function("Parallel Sum (Medium)", |b| b.iter(|| parallel_sum(black_box(&medium_data))));
    group.bench_function("Adaptive Sum (Medium)", |b| b.iter(|| adaptive_sum(black_box(&medium_data))));

    group.bench_function("Normal Sum (Large)", |b| b.iter(|| normal_sum(black_box(&large_data))));
    group.bench_function("Parallel Sum (Large)", |b| b.iter(|| parallel_sum(black_box(&large_data))));
    group.bench_function("Adaptive Sum (Large)", |b| b.iter(|| adaptive_sum(black_box(&large_data))));

    // Count benchmarks
    group.bench_function("Normal Count (Large)", |b| b.iter(|| normal_count(black_box(&large_data))));
    group.bench_function("Parallel Count (Large)", |b| b.iter(|| parallel_count(black_box(&large_data))));
    group.bench_function("Adaptive Count (Large)", |b| b.iter(|| adaptive_count(black_box(&large_data))));

    // Average benchmarks
    group.bench_function("Normal Avg (Large)", |b| b.iter(|| normal_avg(black_box(&large_data))));
    group.bench_function("Parallel Avg (Large)", |b| b.iter(|| parallel_avg(black_box(&large_data))));
    group.bench_function("Adaptive Avg (Large)", |b| b.iter(|| adaptive_avg(black_box(&large_data))));

    // Min benchmarks
    group.bench_function("Normal Min (Large)", |b| b.iter(|| normal_min(black_box(&large_data))));
    group.bench_function("Parallel Min (Large)", |b| b.iter(|| parallel_min(black_box(&large_data))));
    group.bench_function("Adaptive Min (Large)", |b| b.iter(|| adaptive_min(black_box(&large_data))));

    // Max benchmarks
    group.bench_function("Normal Max (Large)", |b| b.iter(|| normal_max(black_box(&large_data))));
    group.bench_function("Parallel Max (Large)", |b| b.iter(|| parallel_max(black_box(&large_data))));
    group.bench_function("Adaptive Max (Large)", |b| b.iter(|| adaptive_max(black_box(&large_data))));

    // Distinct Count benchmarks
    group.bench_function("Normal Distinct Count (Large)", |b| b.iter(|| normal_distinct_count(black_box(&large_data))));
    group.bench_function("Parallel Distinct Count (Large)", |b| b.iter(|| parallel_distinct_count(black_box(&large_data))));
    group.bench_function("Adaptive Distinct Count (Large)", |b| b.iter(|| adaptive_distinct_count(black_box(&large_data))));
    
    group.finish();
}

criterion_group!(benches, bench_aggregations);
criterion_main!(benches);