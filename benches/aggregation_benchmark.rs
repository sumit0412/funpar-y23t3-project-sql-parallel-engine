use criterion::{black_box, criterion_group, criterion_main, Criterion};
use project::aggregations::{normal_sum, parallel_sum, parallel_sum_chunked};

fn bench_sum(c: &mut Criterion) {
    let data: Vec<i32> = (0..1_000_000).collect();

    let mut group = c.benchmark_group("Sum Operations");

    group.bench_function("Normal Sum", |b| {
        b.iter(|| normal_sum(black_box(&data)))
    });

    group.bench_function("Parallel Sum", |b| {
        b.iter(|| parallel_sum(black_box(&data)))
    });

    for &chunk_size in &[1000, 10000, 100000] {
        group.bench_function(format!("Parallel Sum Chunked (size {})", chunk_size), |b| {
            b.iter(|| parallel_sum_chunked(black_box(&data), chunk_size))
        });
    }

    group.finish();
}

criterion_group!(benches, bench_sum);
criterion_main!(benches);