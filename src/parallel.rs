use rayon::prelude::*;

pub fn parallel_scan<T, F>(data: &[T], process: F) -> Vec<T>
where
    T: Send + Sync + Clone,
    F: Fn(&T) -> T + Sync + Send,
{
    data.par_iter().map(process).collect()
}