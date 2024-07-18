use criterion::{black_box, criterion_group, criterion_main, Criterion};
use project::sorting::{sequential_merge_sort, parallel_merge_sort};
use rand::Rng;

fn generate_random_data(size: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..size).map(|_| rng.gen_range(1..1000)).collect()
}

fn bench_sorting(c: &mut Criterion) {
    let data_sizes = [1000, 10_000, 100_000, 1_000_000];

    for size in data_sizes.iter() {
        let data: Vec<i32> = generate_random_data(*size);

        let mut group = c.benchmark_group(format!("Sorting (size {})", size));

        group.bench_function("Sequential Merge Sort", |b| {
            b.iter(|| sequential_merge_sort(black_box(&data)))
        });

        group.bench_function("Parallel Merge Sort", |b| {
            b.iter(|| parallel_merge_sort(black_box(&data)))
        });

        group.finish();
    }
}

criterion_group!(benches, bench_sorting);
criterion_main!(benches);