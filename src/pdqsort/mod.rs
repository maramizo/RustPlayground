use rayon::prelude::*;

/// Serial Pattern Defeating Quick Sort
///
/// Takes an array, copies it, sorts it, and returns true if the array is sorted.
///
/// # Test
///
/// ```
/// use rust_playground::pdqsort::serial_pdqsort;
/// assert_eq!(serial_pdqsort(&vec![-3, 315908580, -31490314, 321, 0]), vec![-31490314, -3, 0, 321, 315908580]);
/// ```
pub fn serial_pdqsort(arr: &Vec<i32>) -> Vec<i32> {
    let mut arr = arr.clone();
    let now = std::time::Instant::now();
    arr.sort_unstable();
    println!("Single pdqsort time: {}ms", now.elapsed().as_millis());
    arr
}


/// Parallel Pattern Defeating Quick Sort
///
/// Takes an array, copies it, sorts it, and returns true if the array is sorted, however,
/// it uses rayon to parallelize the sorting.
///
/// # Test
///
/// ```
/// use rust_playground::pdqsort::parallel_pdqsort;
/// assert_eq!(parallel_pdqsort(&vec![-3, 315908580, -31490314, 321, 0]), vec![-31490314, -3, 0, 321, 315908580]);
/// ```
pub fn parallel_pdqsort(arr: &Vec<i32>) -> Vec<i32> {
    let mut arr = arr.clone();
    let now = std::time::Instant::now();
    arr.par_sort_unstable();
    println!("Parallel rayon time: {}ms", now.elapsed().as_millis());
    arr
}