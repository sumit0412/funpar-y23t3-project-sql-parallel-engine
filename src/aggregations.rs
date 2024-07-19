use rayon::prelude::*;
use std::collections::HashSet;

const PARALLELIZATION_THRESHOLD: usize = 50000;

fn should_parallelize(data_size: usize) -> bool {
    data_size > PARALLELIZATION_THRESHOLD
}

pub fn adaptive_sum(data: &[i32]) -> (i32, String) {
    if should_parallelize(data.len()) {
        (parallel_sum(data), "Parallel".to_string())
    } else {
        (normal_sum(data), "Sequential".to_string())
    }
}

pub fn adaptive_count(data: &[i32]) -> (usize, String) {
    // Count is always faster sequentially based on the benchmark results
    (normal_count(data), "Sequential".to_string())
}

pub fn adaptive_avg(data: &[i32]) -> (f64, String) {
    if should_parallelize(data.len()) {
        (parallel_avg(data), "Parallel".to_string())
    } else {
        (normal_avg(data), "Sequential".to_string())
    }
}

pub fn adaptive_min(data: &[i32]) -> (i32, String) {
    if should_parallelize(data.len()) {
        (parallel_min(data), "Parallel".to_string())
    } else {
        (normal_min(data), "Sequential".to_string())
    }
}

pub fn adaptive_max(data: &[i32]) -> (i32, String) {
    if should_parallelize(data.len()) {
        (parallel_max(data), "Parallel".to_string())
    } else {
        (normal_max(data), "Sequential".to_string())
    }
}

pub fn adaptive_distinct_count(data: &[i32]) -> (usize, String) {
    if should_parallelize(data.len()) {
        (parallel_distinct_count(data), "Parallel".to_string())
    } else {
        (normal_distinct_count(data), "Sequential".to_string())
    }
}

pub fn normal_sum(data: &[i32]) -> i32 {
    data.iter().sum()
}

pub fn parallel_sum(data: &[i32]) -> i32 {
    data.par_iter().sum()
}

pub fn normal_count(data: &[i32]) -> usize {
    data.len()
}

pub fn parallel_count(data: &[i32]) -> usize {
    data.par_iter().count()
}

pub fn normal_avg(data: &[i32]) -> f64 {
    let sum: i32 = data.iter().sum();
    sum as f64 / data.len() as f64
}

pub fn parallel_avg(data: &[i32]) -> f64 {
    let (sum, count) = data.par_iter()
        .map(|&x| (x as i64, 1usize))
        .reduce(
            || (0, 0),
            |(sum1, count1), (sum2, count2)| (sum1 + sum2, count1 + count2)
        );
    sum as f64 / count as f64
}

pub fn normal_min(data: &[i32]) -> i32 {
    *data.iter().min().unwrap_or(&i32::MAX)
}

pub fn parallel_min(data: &[i32]) -> i32 {
    data.par_iter().copied().min().unwrap_or(i32::MAX)
}

pub fn normal_max(data: &[i32]) -> i32 {
    *data.iter().max().unwrap_or(&i32::MIN)
}

pub fn parallel_max(data: &[i32]) -> i32 {
    data.par_iter().copied().max().unwrap_or(i32::MIN)
}

pub fn normal_distinct_count(data: &[i32]) -> usize {
    let mut set = HashSet::new();
    data.iter().for_each(|&x| {
        set.insert(x);
    });
    set.len()
}

pub fn parallel_distinct_count(data: &[i32]) -> usize {
    data.par_chunks(data.len() / rayon::current_num_threads().max(1))
        .map(|chunk| {
            let mut set = HashSet::new();
            chunk.iter().for_each(|&x| {
                set.insert(x);
            });
            set
        })
        .reduce(
            || HashSet::new(),
            |mut acc, x| {
                acc.extend(x);
                acc
            },
        )
        .len()
}