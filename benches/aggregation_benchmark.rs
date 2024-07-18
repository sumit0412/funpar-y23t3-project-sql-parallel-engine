use criterion::{black_box, criterion_group, criterion_main, Criterion};
use project::aggregations::*;
use rand::Rng;

fn generate_random_data(size: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..size).map(|_| rng.gen_range(1..1000)).collect()
}

fn bench_aggregations(c: &mut Criterion) {
    let data: Vec<i32> = generate_random_data(1_000_000);

    let mut group = c.benchmark_group("Aggregation Operations");

    group.bench_function("Normal Sum", |b| {
        b.iter(|| normal_sum(black_box(&data)))
    });

    group.bench_function("Parallel Sum", |b| {
        b.iter(|| parallel_sum(black_box(&data)))
    });

    group.bench_function("Normal Count", |b| {
        b.iter(|| normal_count(black_box(&data)))
    });

    group.bench_function("Parallel Count", |b| {
        b.iter(|| parallel_count(black_box(&data)))
    });

    group.bench_function("Normal Avg", |b| {
        b.iter(|| normal_avg(black_box(&data)))
    });

    group.bench_function("Parallel Avg", |b| {
        b.iter(|| parallel_avg(black_box(&data)))
    });

    group.bench_function("Normal Min", |b| {
        b.iter(|| normal_min(black_box(&data)))
    });

    group.bench_function("Parallel Min", |b| {
        b.iter(|| parallel_min(black_box(&data)))
    });

    group.bench_function("Normal Max", |b| {
        b.iter(|| normal_max(black_box(&data)))
    });

    group.bench_function("Parallel Max", |b| {
        b.iter(|| parallel_max(black_box(&data)))
    });

    group.bench_function("Normal Distinct Count", |b| {
        b.iter(|| normal_distinct_count(black_box(&data)))
    });

    group.bench_function("Parallel Distinct Count", |b| {
        b.iter(|| parallel_distinct_count(black_box(&data)))
    });

    group.bench_function("Normal Median", |b| {
        b.iter(|| normal_median(black_box(&data)))
    });

    group.bench_function("Parallel Median", |b| {
        b.iter(|| parallel_median(black_box(&data)))
    });

    for &chunk_size in &[1000, 10000, 100000] {
        group.bench_function(format!("Parallel Sum Chunked (size {})", chunk_size), |b| {
            b.iter(|| parallel_sum_chunked(black_box(&data), chunk_size))
        });

        group.bench_function(format!("Parallel Avg Chunked (size {})", chunk_size), |b| {
            b.iter(|| parallel_avg_chunked(black_box(&data), chunk_size))
        });

        group.bench_function(format!("Parallel Min Chunked (size {})", chunk_size), |b| {
            b.iter(|| parallel_min_chunked(black_box(&data), chunk_size))
        });

        group.bench_function(format!("Parallel Max Chunked (size {})", chunk_size), |b| {
            b.iter(|| parallel_max_chunked(black_box(&data), chunk_size))
        });

        group.bench_function(format!("Parallel Distinct Count Chunked (size {})", chunk_size), |b| {
            b.iter(|| parallel_distinct_count_chunked(black_box(&data), chunk_size))
        });

        group.bench_function(format!("Parallel Median Chunked (size {})", chunk_size), |b| {
            b.iter(|| parallel_median_chunked(black_box(&data), chunk_size))
        });
    }

    group.finish();
}

criterion_group!(benches, bench_aggregations);
criterion_main!(benches);