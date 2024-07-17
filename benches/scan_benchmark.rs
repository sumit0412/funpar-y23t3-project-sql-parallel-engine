use criterion::{black_box, criterion_group, criterion_main, Criterion};

use project::parallel::{
    normal_scan, normal_scan_with_filter, parallel_scan, parallel_scan_chunked,
    parallel_scan_chunked_with_filter, parallel_scan_with_filter,
};

fn process(x: &i32) -> i32 {
    (0..100).fold(*x, |acc: i32, _| acc.wrapping_add(1))
}

fn predicate(x: &i32) -> bool {
    x % 2 == 0
}

fn bench_scans(c: &mut Criterion) {
    let data: Vec<i32> = (0..10_000).collect();

    let mut group = c.benchmark_group("Scan Operations");

    group.bench_function("Normal Scan", |b| {
        b.iter(|| normal_scan(black_box(&data), process))
    });

    group.bench_function("Parallel Scan", |b| {
        b.iter(|| parallel_scan(black_box(&data), process))
    });

    group.bench_function("Normal Scan with Filter", |b| {
        b.iter(|| normal_scan_with_filter(black_box(&data), process, predicate))
    });

    group.bench_function("Parallel Scan with Filter", |b| {
        b.iter(|| parallel_scan_with_filter(black_box(&data), process, predicate))
    });

    // group.bench_function("Parallel Scan with Count", |b| {
    //     b.iter(|| parallel_scan_with_count(black_box(&data), process))
    // });

    for &chunk_size in &[100, 1000, 10000, 100000] {
        group.bench_function(format!("Parallel Chunked (size {})", chunk_size), |b| {
            b.iter(|| parallel_scan_chunked(black_box(&data), process, chunk_size))
        });

        group.bench_function(
            format!("Parallel Chunked with Filter (size {})", chunk_size),
            |b| {
                b.iter(|| {
                    parallel_scan_chunked_with_filter(
                        black_box(&data),
                        process,
                        predicate,
                        chunk_size,
                    )
                })
            },
        );
    }

    group.finish();
}

criterion_group!(benches, bench_scans);
criterion_main!(benches);