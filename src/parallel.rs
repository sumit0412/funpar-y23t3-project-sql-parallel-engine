use rayon::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};

static POOL_USES: AtomicUsize = AtomicUsize::new(0);

pub fn normal_scan<T, F>(data: &[T], process: F) -> Vec<T>
where
    T: Clone,
    F: Fn(&T) -> T,
{
    data.iter().map(process).collect()
}

// This is the basic parallel scan that uses process
pub fn parallel_scan<T, F>(data: &[T], process: F) -> Vec<T>
where
    T: Send + Sync + Clone,
    F: Fn(&T) -> T + Sync + Send,
{
    data.par_iter().map(process).collect()
}

// This counts the number of times the thread pool was used
pub fn parallel_scan_with_count<T, F>(data: &[T], process: F) -> Vec<T>
where
    T: Send + Sync + Clone,
    F: Fn(&T) -> T + Sync + Send,
{
    POOL_USES.store(0, Ordering::SeqCst);
    let result = data.par_iter().map(|item| {
        POOL_USES.fetch_add(1, Ordering::SeqCst);
        process(item)
    }).collect();
    println!("Thread pool was used {} times", POOL_USES.load(Ordering::SeqCst));
    result
}

// This is a new aproach using chunks, to figure out which chunk size is the best
pub fn parallel_scan_chunked<T, F>(data: &[T], process: F, chunk_size: usize) -> Vec<T>
where
    T: Send + Sync + Clone,
    F: Fn(&T) -> T + Sync + Send,
{
    data.par_chunks(chunk_size)
        .flat_map(|chunk| chunk.par_iter().map(&process))
        .collect()
}