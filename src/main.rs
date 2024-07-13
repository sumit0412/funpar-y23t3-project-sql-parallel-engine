mod parallel;

use std::time::Instant;
use parallel::{normal_scan, parallel_scan};

fn main() {
    let data: Vec<i32> = (0..100).collect();

    // Measuring the normal code execution time
    let start = Instant::now();
    let result_normal = normal_scan(&data, |x| x + 1);
    let duration_normal = start.elapsed();
    println!("Normal execution time: {:?}", duration_normal);

    // Measuring the parallel code execution time
    let start = Instant::now();
    let result_parallel = parallel_scan(&data, |x| x + 1);
    let duration_parallel = start.elapsed();
    println!("Parallel execution time: {:?}", duration_parallel);

    // Making sure that both methods produce the same result because parallel code might have data race or some other error which can affect the result
    assert_eq!(result_normal, result_parallel);
}
