use rayon::prelude::*;

// Adaptive chunk size determination
fn determine_chunk_size(data_size: usize) -> usize {
    let available_threads = rayon::current_num_threads();
    let base_chunk_size = 10000; // Increased based on benchmark results
    let chunks_per_thread = 2; // Reduced to increase chunk size
    let ideal_chunks = available_threads * chunks_per_thread;
    std::cmp::max(base_chunk_size, data_size / ideal_chunks)
}

// Adaptive parallelism decision
fn should_parallelize(data_size: usize) -> bool {
    let threshold = 50000; // Increased based on benchmark results
    data_size > threshold
}

#[allow(dead_code)]
pub fn adaptive_scan<T, F>(data: &[T], process: F) -> Vec<T>
where
    T: Send + Sync + Clone,
    F: Fn(&T) -> T + Sync + Send,
{
    if should_parallelize(data.len()) {
        let chunk_size = determine_chunk_size(data.len());
        parallel_scan_chunked(data, process, chunk_size)
    } else {
        normal_scan(data, process)
    }
}

// Adaptive scan with filter function
#[allow(dead_code)]
pub fn adaptive_scan_with_filter<T, F, P>(data: &[T], process: F, predicate: P) -> Vec<T>
where
    T: Send + Sync + Clone,
    F: Fn(&T) -> T + Sync + Send,
    P: Fn(&T) -> bool + Sync + Send,
{
    if should_parallelize(data.len()) {
        let chunk_size = determine_chunk_size(data.len());
        parallel_scan_chunked_with_filter(data, process, predicate, chunk_size)
    } else {
        normal_scan_with_filter(data, process, predicate)
    }
}

#[allow(dead_code)]
pub fn normal_scan<T, F>(data: &[T], process: F) -> Vec<T>
where
    T: Clone,
    F: Fn(&T) -> T,
{
    data.iter().map(process).collect()
}

#[allow(dead_code)]
pub fn parallel_scan<T, F>(data: &[T], process: F) -> Vec<T>
where
    T: Send + Sync + Clone,
    F: Fn(&T) -> T + Sync + Send,
{
    data.par_iter().map(process).collect()
}

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