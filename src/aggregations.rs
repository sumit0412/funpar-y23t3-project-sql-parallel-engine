use rayon::prelude::*;
use std::cmp::{max, min};
use std::collections::HashSet;
use std::hash::Hash;
use std::ops::Add;

pub fn normal_sum<T>(data: &[T]) -> T
where
    T: Clone + Add<Output = T> + Default,
{
    data.iter().cloned().fold(T::default(), |acc, x| acc + x)
}

pub fn parallel_sum<T>(data: &[T]) -> T
where
    T: Send + Sync + Clone + Add<Output = T> + Default,
{
    data.par_iter().cloned().reduce(|| T::default(), |a, b| a + b)
}

pub fn parallel_sum_chunked<T>(data: &[T], chunk_size: usize) -> T
where
    T: Send + Sync + Clone + Add<Output = T> + Default,
{
    data.par_chunks(chunk_size)
        .map(|chunk| chunk.iter().cloned().fold(T::default(), |acc, x| acc + x))
        .reduce(|| T::default(), |a, b| a + b)
}

pub fn normal_count<T>(data: &[T]) -> usize {
    data.len()
}

pub fn parallel_count<T: Send + Sync>(data: &[T]) -> usize {
    data.par_iter().count()
}

pub fn normal_avg<T>(data: &[T]) -> f64
where
    T: Clone + Into<f64>,
{
    let sum: f64 = data.iter().map(|x| (*x).clone().into()).sum();
    sum / data.len() as f64
}

pub fn parallel_avg<T>(data: &[T]) -> f64
where
    T: Send + Sync + Clone + Into<f64>,
{
    let (sum, count) = data.par_iter().map(|x| ((*x).clone().into(), 1u64)).reduce(
        || (0.0, 0),
        |(sum1, count1), (sum2, count2)| (sum1 + sum2, count1 + count2),
    );
    sum / count as f64
}

pub fn normal_min<T: Ord + Clone>(data: &[T]) -> Option<T> {
    data.iter().min().cloned()
}

pub fn parallel_min<T: Ord + Clone + Send + Sync>(data: &[T]) -> Option<T> {
    data.par_iter().min().cloned()
}

pub fn normal_max<T: Ord + Clone>(data: &[T]) -> Option<T> {
    data.iter().max().cloned()
}

pub fn parallel_max<T: Ord + Clone + Send + Sync>(data: &[T]) -> Option<T> {
    data.par_iter().max().cloned()
}

pub fn normal_distinct_count<T: Eq + Hash + Clone>(data: &[T]) -> usize {
    data.iter().cloned().collect::<HashSet<_>>().len()
}

pub fn parallel_distinct_count<T: Eq + Hash + Clone + Send + Sync>(data: &[T]) -> usize {
    data.par_iter()
        .map(|x| {
            let mut set = HashSet::new();
            set.insert(x.clone());
            set
        })
        .reduce(HashSet::new, |mut acc, set| {
            acc.extend(set);
            acc
        })
        .len()
}

pub fn normal_median<T: Ord + Clone>(data: &[T]) -> Option<T> {
    if data.is_empty() {
        return None;
    }
    let mut sorted_data = data.to_vec();
    sorted_data.sort();
    let mid = sorted_data.len() / 2;
    Some(sorted_data[mid].clone())
}

pub fn parallel_median<T: Ord + Clone + Send + Sync>(data: &[T]) -> Option<T> {
    if data.is_empty() {
        return None;
    }
    let mut sorted_data: Vec<T> = data.to_vec();
    sorted_data.par_sort();
    let mid = sorted_data.len() / 2;
    Some(sorted_data[mid].clone())
}

pub fn parallel_avg_chunked<T>(data: &[T], chunk_size: usize) -> f64
where
    T: Send + Sync + Clone + Into<f64>,
{
    let (sum, count) = data
        .par_chunks(chunk_size)
        .map(|chunk| {
            let sum: f64 = chunk.iter().map(|x| (*x).clone().into()).sum();
            (sum, chunk.len() as u64)
        })
        .reduce(
            || (0.0, 0),
            |(sum1, count1), (sum2, count2)| (sum1 + sum2, count1 + count2),
        );
    sum / count as f64
}

pub fn parallel_min_chunked<T: Ord + Clone + Send + Sync>(data: &[T], chunk_size: usize) -> Option<T> {
    data.par_chunks(chunk_size)
        .map(|chunk| chunk.iter().min().cloned())
        .reduce(|| None, |a, b| match (a, b) {
            (Some(x), Some(y)) => Some(min(x, y)),
            (Some(x), None) => Some(x),
            (None, Some(y)) => Some(y),
            (None, None) => None,
        })
}

pub fn parallel_max_chunked<T: Ord + Clone + Send + Sync>(data: &[T], chunk_size: usize) -> Option<T> {
    data.par_chunks(chunk_size)
        .map(|chunk| chunk.iter().max().cloned())
        .reduce(|| None, |a, b| match (a, b) {
            (Some(x), Some(y)) => Some(max(x, y)),
            (Some(x), None) => Some(x),
            (None, Some(y)) => Some(y),
            (None, None) => None,
        })
}

pub fn parallel_distinct_count_chunked<T: Eq + Hash + Clone + Send + Sync>(data: &[T], chunk_size: usize) -> usize {
    data.par_chunks(chunk_size)
        .map(|chunk| chunk.iter().cloned().collect::<HashSet<_>>())
        .reduce(HashSet::new, |mut acc, set| {
            acc.extend(set);
            acc
        })
        .len()
}

pub fn parallel_median_chunked<T: Ord + Clone + Send + Sync>(data: &[T], chunk_size: usize) -> Option<T> {
    if data.is_empty() {
        return None;
    }
    
    let mut sorted_chunks: Vec<Vec<T>> = data
        .par_chunks(chunk_size)
        .map(|chunk| {
            let mut chunk_vec = chunk.to_vec();
            chunk_vec.sort();
            chunk_vec
        })
        .collect();

    // Merge sorted chunks
    while sorted_chunks.len() > 1 {
        sorted_chunks = sorted_chunks
            .par_chunks(2)
            .map(|pair| match pair {
                [left, right] => {
                    let mut merged = Vec::with_capacity(left.len() + right.len());
                    let mut left_iter = left.iter();
                    let mut right_iter = right.iter();
                    let mut left_next = left_iter.next();
                    let mut right_next = right_iter.next();
                    
                    while left_next.is_some() || right_next.is_some() {
                        match (left_next, right_next) {
                            (Some(l), Some(r)) => {
                                if l <= r {
                                    merged.push(l.clone());
                                    left_next = left_iter.next();
                                } else {
                                    merged.push(r.clone());
                                    right_next = right_iter.next();
                                }
                            }
                            (Some(l), None) => {
                                merged.push(l.clone());
                                left_next = left_iter.next();
                            }
                            (None, Some(r)) => {
                                merged.push(r.clone());
                                right_next = right_iter.next();
                            }
                            (None, None) => break,
                        }
                    }
                    merged
                }
                [single] => single.clone(),
                _ => unreachable!(),
            })
            .collect();
    }

    let final_sorted = sorted_chunks.into_iter().next().unwrap();
    let mid = final_sorted.len() / 2;
    Some(final_sorted[mid].clone())
}