use std::hash::Hash;
use std::collections::HashMap;
use rayon::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct GroupByResult<K, V> {
    pub key: K,
    pub avg_value: V,
}

pub fn sequential_group_by<T, K, F>(data: &[T], key_func: F) -> Vec<GroupByResult<K, f64>>
where
    T: Clone,
    K: Eq + Hash + Clone,
    F: Fn(&T) -> (K, f64),
{
    let mut groups: HashMap<K, (f64, usize)> = HashMap::new();

    for item in data {
        let (key, value) = key_func(item);
        let entry = groups.entry(key).or_insert((0.0, 0));
        entry.0 += value;
        entry.1 += 1;
    }

    groups
        .into_iter()
        .map(|(key, (sum, count))| GroupByResult { 
            key, 
            avg_value: sum / count as f64 
        })
        .collect()
}

pub fn parallel_group_by<T, K, F>(data: &[T], key_func: F) -> Vec<GroupByResult<K, f64>>
where
    T: Clone + Send + Sync,
    K: Eq + Hash + Clone + Send,
    F: Fn(&T) -> (K, f64) + Sync + Send,
{
    let grouped: HashMap<K, (f64, usize)> = data
        .par_iter()
        .map(|item| {
            let (key, value) = key_func(item);
            (key, (value, 1))
        })
        .fold(
            || HashMap::new(),
            |mut acc, (key, (value, count))| {
                let entry = acc.entry(key).or_insert((0.0, 0));
                entry.0 += value;
                entry.1 += count;
                acc
            },
        )
        .reduce(
            || HashMap::new(),
            |mut a, b| {
                for (k, (sum, count)) in b {
                    let entry = a.entry(k).or_insert((0.0, 0));
                    entry.0 += sum;
                    entry.1 += count;
                }
                a
            },
        );

    grouped
        .into_par_iter()
        .map(|(key, (sum, count))| GroupByResult { 
            key, 
            avg_value: sum / count as f64 
        })
        .collect()
}