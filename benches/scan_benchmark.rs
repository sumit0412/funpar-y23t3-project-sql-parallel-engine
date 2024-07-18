use criterion::{black_box, criterion_group, criterion_main, Criterion};

use project::scans::{
    adaptive_scan, adaptive_scan_with_filter, normal_scan, normal_scan_with_filter, parallel_scan,
    parallel_scan_chunked, parallel_scan_chunked_with_filter, parallel_scan_with_filter,
};

fn process(x: &i32) -> i32 {
    (0..100).fold(*x, |acc: i32, _| acc.wrapping_add(1))
}

fn predicate(x: &i32) -> bool {
    x % 2 == 0
}

fn bench_scans(c: &mut Criterion) {
    let small_data: Vec<i32> = (0..10_000).collect();
    let medium_data: Vec<i32> = (0..100_000).collect();
    let large_data: Vec<i32> = (0..1_000_000).collect();

    let mut group = c.benchmark_group("Scan Operations");

    // Small data benchmarks
    group.bench_function("Normal Scan (Small)", |b| {
        b.iter(|| normal_scan(black_box(&small_data), process))
    });
    group.bench_function("Parallel Scan (Small)", |b| {
        b.iter(|| parallel_scan(black_box(&small_data), process))
    });
    group.bench_function("Adaptive Scan (Small)", |b| {
        b.iter(|| adaptive_scan(black_box(&small_data), process))
    });

    // Medium data benchmarks
    group.bench_function("Normal Scan (Medium)", |b| {
        b.iter(|| normal_scan(black_box(&medium_data), process))
    });
    group.bench_function("Parallel Scan (Medium)", |b| {
        b.iter(|| parallel_scan(black_box(&medium_data), process))
    });
    group.bench_function("Adaptive Scan (Medium)", |b| {
        b.iter(|| adaptive_scan(black_box(&medium_data), process))
    });

    // Large data benchmarks
    group.bench_function("Normal Scan (Large)", |b| {
        b.iter(|| normal_scan(black_box(&large_data), process))
    });
    group.bench_function("Parallel Scan (Large)", |b| {
        b.iter(|| parallel_scan(black_box(&large_data), process))
    });
    group.bench_function("Adaptive Scan (Large)", |b| {
        b.iter(|| adaptive_scan(black_box(&large_data), process))
    });

    // Filter benchmarks
    group.bench_function("Normal Scan with Filter (Large)", |b| {
        b.iter(|| normal_scan_with_filter(black_box(&large_data), process, predicate))
    });
    group.bench_function("Parallel Scan with Filter (Large)", |b| {
        b.iter(|| parallel_scan_with_filter(black_box(&large_data), process, predicate))
    });
    group.bench_function("Adaptive Scan with Filter (Large)", |b| {
        b.iter(|| adaptive_scan_with_filter(black_box(&large_data), process, predicate))
    });

    // Chunked parallel scan benchmarks
    for &chunk_size in &[10000, 50000, 100000, 500000] {
        group.bench_function(format!("Parallel Chunked (size {})", chunk_size), |b| {
            b.iter(|| parallel_scan_chunked(black_box(&large_data), process, chunk_size))
        });
        group.bench_function(format!("Parallel Chunked with Filter (size {})", chunk_size), |b| {
            b.iter(|| parallel_scan_chunked_with_filter(black_box(&large_data), process, predicate, chunk_size))
        });
    }

    group.finish();
}

criterion_group!(benches, bench_scans);
criterion_main!(benches);