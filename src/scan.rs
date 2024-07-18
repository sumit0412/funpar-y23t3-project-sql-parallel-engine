use rayon::prelude::*;

#[allow(dead_code)]
pub fn normal_scan<T, F>(data: &[T], process: F) -> Vec<T>
where
    T: Clone,
    F: Fn(&T) -> T,
{
    data.iter().map(process).collect()
}

// This is the basic parallel scan that uses process
#[allow(dead_code)]
pub fn parallel_scan<T, F>(data: &[T], process: F) -> Vec<T>
where
    T: Send + Sync + Clone,
    F: Fn(&T) -> T + Sync + Send,
{
    data.par_iter().map(process).collect()
}

// This is a new aproach using chunks, to figure out which chunk size is the best
#[allow(dead_code)]
pub fn parallel_scan_chunked<T, F>(data: &[T], process: F, chunk_size: usize) -> Vec<T>
where
    T: Send + Sync + Clone,
    F: Fn(&T) -> T + Sync + Send,
{
    data.par_chunks(chunk_size)
        .flat_map(|chunk| chunk.par_iter().map(&process))
        .collect()
}

#[allow(dead_code)]
pub fn normal_scan_with_filter<T, F, P>(data: &[T], process: F, predicate: P) -> Vec<T>
where
    T: Clone,
    F: Fn(&T) -> T,
    P: Fn(&T) -> bool,
{
    data.iter()
        .filter(|item| predicate(item))
        .map(process)
        .collect()
}

#[allow(dead_code)]
pub fn parallel_scan_with_filter<T, F, P>(data: &[T], process: F, predicate: P) -> Vec<T>
where
    T: Send + Sync + Clone,
    F: Fn(&T) -> T + Sync + Send,
    P: Fn(&T) -> bool + Sync + Send,
{
    data.par_iter()
        .filter(|item| predicate(item))
        .map(process)
        .collect()
}

#[allow(dead_code)]
pub fn parallel_scan_chunked_with_filter<T, F, P>(
    data: &[T],
    process: F,
    predicate: P,
    chunk_size: usize,
) -> Vec<T>
where
    T: Send + Sync + Clone,
    F: Fn(&T) -> T + Sync + Send,
    P: Fn(&T) -> bool + Sync + Send,
{
    data.par_chunks(chunk_size)
        .flat_map(|chunk| {
            chunk
                .par_iter()
                .filter(|item| predicate(item))
                .map(&process)
        })
        .collect()
}