use rayon::prelude::*;

fn merge_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }
    let mid = arr.len() / 2;
    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);
    merge(arr, mid);
}

fn merge(arr: &mut [i32], mid: usize) {
    let left = arr[..mid].to_vec();
    let right = arr[mid..].to_vec();
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            arr[k] = left[i];
            i += 1;
        } else {
            arr[k] = right[j];
            j += 1;
        }
        k += 1;
    }
    while i < left.len() {
        arr[k] = left[i];
        i += 1;
        k += 1;
    }
    while j < right.len() {
        arr[k] = right[j];
        j += 1;
        k += 1;
    }
}

/// Serial mergesort
///
/// # Test
///
/// ```
/// use rust_playground::merge_sort::serial_mergesort;
/// assert_eq!(serial_mergesort(&vec![-3, 315908580, -31490314, 321, 0]), vec![-31490314, -3, 0, 321, 315908580]);
/// ```
pub fn serial_mergesort(arr: &Vec<i32>) -> Vec<i32> {
    let mut arr = arr.clone();
    let now = std::time::Instant::now();
    merge_sort(&mut arr);
    println!("Serial mergesort time: {}ms", now.elapsed().as_millis());
    arr
}

/// Parallel mergesort
///
/// # Test
///
/// ```
/// use rust_playground::merge_sort::parallel_mergesort;
/// assert_eq!(parallel_mergesort(&vec![-3, 315908580, -31490314, 321, 0]), vec![-31490314, -3, 0, 321, 315908580]);
/// ```
pub fn parallel_mergesort(arr: &Vec<i32>) -> Vec<i32> {
    let mut arr = arr.clone();
    let array_size: usize = arr.len();
    let mut chunk_size: usize = std::cmp::max(array_size / std::thread::available_parallelism().unwrap(), 1);
    let now = std::time::Instant::now();
    arr.par_chunks_mut(chunk_size)
        .for_each(|chunk| merge_sort(chunk));

    while chunk_size < array_size {
        let mut i = 0;
        while i < array_size {
            let end = std::cmp::min(i + 2 * chunk_size, array_size);
            let mid = std::cmp::min(i + chunk_size, end);
            merge(&mut arr[i..end], mid - i);
            i += 2 * chunk_size;
        }
        chunk_size *= 2;
    }
    println!("Parallel mergesort time: {}ms", now.elapsed().as_millis());
    arr
}