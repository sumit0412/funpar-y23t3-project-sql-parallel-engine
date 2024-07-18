use rayon::prelude::*;
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