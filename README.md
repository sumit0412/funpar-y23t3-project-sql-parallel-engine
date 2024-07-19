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

## Features

- Adaptive parallelism based on input size and available resources
- Support for various SQL operations including scans, aggregations, sorting, joins, and group by
- Benchmarking tests to compare performance of parallel and sequential implementations

## Future Work

- Implement a query planner and executor
- Add support for more complex SQL queries
- Optimize memory usage for large datasets and adjust my parallel implementation