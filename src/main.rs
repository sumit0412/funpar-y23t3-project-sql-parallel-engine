mod parallel;

use std::time::Instant;
use parallel::{normal_scan, parallel_scan, parallel_scan_chunked, parallel_scan_with_count};

fn main() {
    let data: Vec<i32> = (0..1_000_000).collect();
    
    let process = |x: &i32| {
        // Simulate more complex operation
        (0..100).fold(*x, |acc: i32, _| acc.wrapping_add(1))
    };

    // Measuring the normal code execution time
    let start = Instant::now();
    let result_normal = normal_scan(&data, process);
    let duration_normal = start.elapsed();
    println!("Normal execution time: {:?}", duration_normal);

    // Measuring the parallel code execution time
    let start = Instant::now();
    let result_parallel = parallel_scan(&data, process);
    let duration_parallel = start.elapsed();
    println!("Parallel execution time: {:?}", duration_parallel);

    // Verify results
    assert_eq!(result_normal, result_parallel);

    // Parallel scan with thread pool use count
    let start = Instant::now();
    let result_parallel_count = parallel_scan_with_count(&data, process);
    let duration_parallel_count = start.elapsed();
    println!("Parallel (with count) execution time: {:?}", duration_parallel_count);

    // Chunked parallel scans
    for &chunk_size in &[100, 1000, 10000, 100000] {
        let start = Instant::now();
        let result_chunked = parallel_scan_chunked(&data, process, chunk_size);
        let duration_chunked = start.elapsed();
        println!("Parallel (chunk size {}): {:?}", chunk_size, duration_chunked);
        assert_eq!(result_normal, result_chunked); // Verify result for each chunk size
    }

    // Verify all results are the same
    assert_eq!(result_normal, result_parallel_count);
}
