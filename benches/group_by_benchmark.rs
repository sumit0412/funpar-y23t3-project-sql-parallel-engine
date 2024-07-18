use criterion::{black_box, criterion_group, criterion_main, Criterion};
use project::group_by::{sequential_group_by, parallel_group_by, adaptive_group_by};
use rand::Rng;

#[derive(Clone, Debug)]
struct Record {
    id: i32,
    value: f64,
}

fn generate_random_data(size: usize) -> Vec<Record> {
    let mut rng = rand::thread_rng();
    (0..size)
        .map(|_| Record {
            id: rng.gen_range(1..100),
            value: rng.gen(),
        })
        .collect()
}

fn bench_group_by(c: &mut Criterion) {
    let data_sizes = [1000, 10_000, 100_000, 1_000_000];

    for size in data_sizes.iter() {
        let data = generate_random_data(*size);

        let mut group = c.benchmark_group(format!("Group By (size {})", size));

        group.bench_function("Sequential Group By", |b| {
            b.iter(|| sequential_group_by(black_box(&data), |r| (r.id, r.value)))
        });

        group.bench_function("Parallel Group By", |b| {
            b.iter(|| parallel_group_by(black_box(&data), |r| (r.id, r.value)))
        });

        group.bench_function("Adaptive Group By", |b| {
            b.iter(|| adaptive_group_by(black_box(&data), |r| (r.id, r.value)))
        });

        group.finish();
    }
}

criterion_group!(benches, bench_group_by);
criterion_main!(benches);