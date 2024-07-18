pub fn sequential_merge_sort<T: Ord + Clone>(arr: &[T]) -> Vec<T> {
    if arr.len() <= 1 {
        return arr.to_vec();
    }

    let mid = arr.len() / 2;
    let left = sequential_merge_sort(&arr[..mid]);
    let right = sequential_merge_sort(&arr[mid..]);

    merge(&left, &right)
}

pub fn parallel_merge_sort<T: Ord + Clone + Send + Sync>(arr: &[T]) -> Vec<T> {
    if arr.len() <= 1 {
        return arr.to_vec();
    }

    let mid = arr.len() / 2;
    let (left, right) = rayon::join(
        || parallel_merge_sort(&arr[..mid]),
        || parallel_merge_sort(&arr[mid..]),
    );

    merge(&left, &right)
}

fn merge<T: Ord + Clone>(left: &[T], right: &[T]) -> Vec<T> {
    let mut result = Vec::with_capacity(left.len() + right.len());
    let mut left_iter = left.iter();
    let mut right_iter = right.iter();
    let mut left_peek = left_iter.next();
    let mut right_peek = right_iter.next();

    while left_peek.is_some() || right_peek.is_some() {
        if right_peek.is_none() || (left_peek.is_some() && left_peek.unwrap() <= right_peek.unwrap()) {
            result.push(left_peek.unwrap().clone());
            left_peek = left_iter.next();
        } else {
            result.push(right_peek.unwrap().clone());
            right_peek = right_iter.next();
        }
    }

    result
}

pub fn sequential_quicksort<T: Ord + Clone>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    let pivot_index = partition(arr);
    let (left, right) = arr.split_at_mut(pivot_index);
    sequential_quicksort(left);
    sequential_quicksort(&mut right[1..]);
}

pub fn parallel_quicksort<T: Ord + Clone + Send>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    let pivot_index = partition(arr);
    let (left, right) = arr.split_at_mut(pivot_index);
    
    rayon::join(
        || parallel_quicksort(left),
        || parallel_quicksort(&mut right[1..])
    );
}

fn partition<T: Ord>(arr: &mut [T]) -> usize {
    let pivot_index = arr.len() / 2;
    arr.swap(pivot_index, arr.len() - 1);
    let mut i = 0;
    
    for j in 0..arr.len() - 1 {
        if arr[j] <= arr[arr.len() - 1] {
            arr.swap(i, j);
            i += 1;
        }
    }
    
    arr.swap(i, arr.len() - 1);
    i
}