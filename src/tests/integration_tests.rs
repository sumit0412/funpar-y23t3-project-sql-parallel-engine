#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallel_scan() {
        let data = vec![1, 2, 3, 4, 5];
        let result = parallel_scan(&data, |x| x * 2);
        assert_eq!(result, vec![2, 4, 6, 8, 10]);
    }
}