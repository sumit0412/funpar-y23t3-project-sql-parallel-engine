use std::collections::HashMap;
use rayon::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Record {
    pub id: i32,
    pub value: String,
}

pub fn sequential_hash_join(left: &[Record], right: &[Record]) -> Vec<(Record, Record)> {
    let mut hash_map: HashMap<i32, Vec<&Record>> = HashMap::new();

    // Build phase
    for record in left {
        hash_map.entry(record.id).or_default().push(record);
    }

    // Probe phase
    let mut result = Vec::new();
    for r_record in right {
        if let Some(l_records) = hash_map.get(&r_record.id) {
            for l_record in l_records {
                result.push(((*l_record).clone(), r_record.clone()));
            }
        }
    }

    result
}

pub fn parallel_hash_join(left: &[Record], right: &[Record]) -> Vec<(Record, Record)> {
    // Build phase
    let hash_map: HashMap<i32, Vec<&Record>> = left
        .par_iter()
        .fold(
            || HashMap::new(),
            |mut acc: HashMap<i32, Vec<&Record>>, record| {
                acc.entry(record.id).or_default().push(record);
                acc
            },
        )
        .reduce(
            || HashMap::new(),
            |mut a, b| {
                for (k, v) in b {
                    a.entry(k).or_default().extend(v);
                }
                a
            },
        );

    // Probe phase
    right
        .par_iter()
        .flat_map(|r_record| {
            hash_map
                .get(&r_record.id)
                .map(|l_records| {
                    l_records
                        .iter()
                        .map(|l_record| ((*l_record).clone(), r_record.clone()))
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default()
        })
        .collect()
}

pub fn sequential_merge_join(left: &[Record], right: &[Record]) -> Vec<(Record, Record)> {
    let mut result = Vec::new();
    let mut i = 0;
    let mut j = 0;

    while i < left.len() && j < right.len() {
        if left[i].id < right[j].id {
            i += 1;
        } else if left[i].id > right[j].id {
            j += 1;
        } else {
            let mut k = j;
            while k < right.len() && right[k].id == left[i].id {
                result.push((left[i].clone(), right[k].clone()));
                k += 1;
            }
            i += 1;
        }
    }

    result
}

pub fn parallel_merge_join(left: &[Record], right: &[Record]) -> Vec<(Record, Record)> {
    let chunk_size = (left.len() / rayon::current_num_threads()).max(1);
    
    left.par_chunks(chunk_size)
        .flat_map(|left_chunk| {
            let mut result = Vec::new();
            let mut j = 0;

            for l_record in left_chunk {
                while j < right.len() && right[j].id < l_record.id {
                    j += 1;
                }
                while j < right.len() && right[j].id == l_record.id {
                    result.push((l_record.clone(), right[j].clone()));
                    j += 1;
                }
            }

            result
        })
        .collect()
}