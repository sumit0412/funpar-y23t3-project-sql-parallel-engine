use criterion::{black_box, criterion_group, criterion_main, Criterion};
use project::joins::{Record, sequential_hash_join, parallel_hash_join, sequential_merge_join, parallel_merge_join};
use rand::{Rng, distributions::Alphanumeric};

fn generate_random_records(size: usize, id_range: std::ops::Range<i32>) -> Vec<Record> {
    let mut rng = rand::thread_rng();
    (0..size)
        .map(|_| Record {
            id: rng.gen_range(id_range.clone()),
            value: (0..10).map(|_| rng.sample(Alphanumeric) as char).collect(),
        })
        .collect()
}

fn bench_joins(c: &mut Criterion) {
    let sizes = [(1000, 1000), (10_000, 10_000), (100_000, 100_000)];

    for (left_size, right_size) in sizes.iter() {
        let left_data = generate_random_records(*left_size, 1..1000);
        let right_data = generate_random_records(*right_size, 1..1000);

        let mut group = c.benchmark_group(format!("Join (left: {}, right: {})", left_size, right_size));

        group.bench_function("Sequential Hash Join", |b| {
            b.iter(|| sequential_hash_join(black_box(&left_data), black_box(&right_data)))
        });

        group.bench_function("Parallel Hash Join", |b| {
            b.iter(|| parallel_hash_join(black_box(&left_data), black_box(&right_data)))
        });

        group.bench_function("Sequential Merge Join", |b| {
            b.iter(|| {
                let mut left = left_data.clone();
                let mut right = right_data.clone();
                left.sort_by_key(|r| r.id);
                right.sort_by_key(|r| r.id);
                sequential_merge_join(black_box(&left), black_box(&right))
            })
        });

        group.bench_function("Parallel Merge Join", |b| {
            b.iter(|| {
                let mut left = left_data.clone();
                let mut right = right_data.clone();
                left.sort_by_key(|r| r.id);
                right.sort_by_key(|r| r.id);
                parallel_merge_join(black_box(&left), black_box(&right))
            })
        });

        group.finish();
    }
}

criterion_group!(benches, bench_joins);
criterion_main!(benches);