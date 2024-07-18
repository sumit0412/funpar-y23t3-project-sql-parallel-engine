use rayon::prelude::*;

// Adaptive chunk size determination
fn determine_chunk_size(data_size: usize) -> usize {
    let available_threads = rayon::current_num_threads();
    let base_chunk_size = 100000; // Adjusted based on the new benchmark results
    let chunks_per_thread = 2;
    let ideal_chunks = available_threads * chunks_per_thread;
    std::cmp::max(base_chunk_size, data_size / ideal_chunks)
}

// Adaptive parallelism decision
fn should_parallelize(data_size: usize) -> bool {
    let threshold = 500000; // Adjusted based on the new benchmark results
    data_size > threshold
}

#[allow(dead_code)]
pub fn adaptive_scan<T, F>(data: &[T], process: F) -> (Vec<T>, String)
where
    T: Send + Sync + Clone,
    F: Fn(&T) -> T + Sync + Send + Clone,
{
    let sequential_result = normal_scan(data, process.clone());
    let parallel_result = parallel_scan(data, process.clone());
    
    if should_parallelize(data.len()) {
        (parallel_result, String::from("Parallel"))
    } else {
        (sequential_result, String::from("Sequential"))
    }
}

#[allow(dead_code)]
pub fn adaptive_scan_with_filter<T, F, P>(data: &[T], process: F, predicate: P) -> (Vec<T>, String)
where
    T: Send + Sync + Clone,
    F: Fn(&T) -> T + Sync + Send + Clone,
    P: Fn(&T) -> bool + Sync + Send + Clone,
{
    let sequential_result = normal_scan_with_filter(data, process.clone(), predicate.clone());
    let parallel_result = parallel_scan_with_filter(data, process.clone(), predicate.clone());
    
    if should_parallelize(data.len()) {
        (parallel_result, String::from("Parallel"))
    } else {
        (sequential_result, String::from("Sequential"))
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
    let chunk_size = determine_chunk_size(data.len());
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
    let chunk_size = determine_chunk_size(data.len());
    data.par_chunks(chunk_size)
        .flat_map(|chunk| {
            chunk
                .par_iter()
                .filter(|item| predicate(item))
                .map(&process)
        })
        .collect()
}