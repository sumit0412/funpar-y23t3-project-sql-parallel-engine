# Parallel SQL Query Execution Database Engine in Rust - Sumit Sachdev

This project implements a parallel SQL query execution engine using Rust, leveraging parallel programming techniques to efficiently process SQL queries on large datasets. The engine is designed to take advantage of Rust's performance and low-level capabilities while optimizing query execution through parallelization.

## Project Overview

The main components of this project include:

1. SQL Parser
2. Parallel Scan Operations
3. Parallel Aggregations
4. Parallel Sorting Algorithms
5. Parallel Join Operations
6. Parallel Group By Operations

Each component is implemented with both sequential and parallel versions, along with an adaptive version that chooses the most efficient method based on the input size and available resources.

## How to Use

Run the integration tests:
cargo test

Run all benchmarks:
cargo bench

## Project Structure

- `src/`: Contains the main source code
    - `parser.rs`: SQL parsing module
    - `scans.rs`: Parallel and sequential scan operations
    - `aggregations.rs`: Parallel and sequential aggregation operations
    - `sorting.rs`: Parallel and sequential sorting algorithms
    - `joins.rs`: Parallel and sequential join operations
    - `group_by.rs`: Parallel and sequential group by operations
- `benches/`: Contains benchmark tests for each operation
- `tests/`: Contains integration tests

## Benchmark Results

### Aggregation Operations (dataset size: 1,000,000)
- SUM: Adaptive (93.782 µs) outperformed Normal (51.824 µs) and Parallel (95.934 µs)
- AVG: Normal (51.998 µs) outperformed Parallel (95.622 µs) and Adaptive (98.077 µs)
- MIN: Parallel (97.873 µs) outperformed Adaptive (99.334 µs) and Normal (699.39 µs)
- MAX: Parallel (95.177 µs) outperformed Adaptive (97.809 µs) and Normal (704.02 µs)
- COUNT: Normal (669.06 ps) outperformed Adaptive (21.433 ns) and Parallel (47.577 µs)
- DISTINCT COUNT: Adaptive (1.9813 ms) outperformed Parallel (2.5020 ms) and Normal (7.9603 ms)

### Group By Operations
- Small dataset (1,000): Sequential (16.593 µs) outperformed Adaptive (17.666 µs) and Parallel (126.23 µs)
- Medium dataset (10,000): Adaptive (119.24 µs) outperformed Sequential (132.79 µs) and Parallel (318.62 µs)
- Large dataset (100,000): Adaptive (967.47 µs) outperformed Sequential (1.0560 ms) and Parallel (1.0575 ms)
- Very Large dataset (1,000,000): Parallel (4.8393 ms) outperformed Adaptive (5.5376 ms) and Sequential (10.047 ms)

### Join Operations (dataset size: 100,000 x 100,000)
- Hash Join: Sequential (1.5808 s) slightly outperformed Parallel (1.7993 s)
- Merge Join: Parallel (38.182 ms) significantly outperformed Sequential (1.6054 s)

### Scan Operations (dataset size: 1,000,000)
- Normal Scan: 320.78 µs
- Parallel Scan: 266.81 µs
- Adaptive Scan: 262.01 µs
- Normal Scan with Filter: 751.28 µs
- Parallel Scan with Filter: 596.00 µs
- Adaptive Scan with Filter: 1.5150 ms

### Sorting Operations (dataset size: 1,000,000)
- Merge Sort: Parallel (33.425 ms) significantly outperformed Sequential (151.54 ms)
- Quicksort: Parallel (61.665 ms) significantly outperformed Sequential (363.33 ms)

## Key Findings

1. Adaptive implementations often perform well, effectiveness of parallelization varies depending on the operation and dataset size.
2. For very large datasets, parallel implementations tend to perform best, especially for complex operations like sorting and joins.
3. Some simple operations (e.g., COUNT) still perform best with normal/sequential implementation.
4. Parallel Merge Join shows dramatic improvement over Sequential Merge Join.
5. Sorting operations benefit significantly from parallelization.
6. Scan operations show mixed results, with parallel scans outperforming normal scans for large datasets without filters, but normal scans performing better with filters.

Parallel is better most of the time, but not all of the time. With further optimizations and fine-tuning, maybe it is possible to improve performance consistently, but with varied datasets, it's always going to be challenging to get it exactly right, especially on different hardware.

## Features

- Adaptive parallelism based on input size and available resources
- Support for various SQL operations including scans, aggregations, sorting, joins, and group by
- Benchmarking tests to compare performance of parallel and sequential implementations

## Future Work

- Implement a query planner and executor
- Add support for more complex SQL queries
- Optimize memory usage for large datasets and adjust my parallel implementation
- Make a way for a user to be able to interact with this engine and make it useful with SQL