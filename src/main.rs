mod merge_sort;
mod pdqsort;
use rand::Rng;
const ARRAY_SIZE: usize = 2_000_000;

fn main(){
    let arr = (0..ARRAY_SIZE).map(|_| rand::thread_rng().gen::<i32>()).collect();
    merge_sort::serial_mergesort(&arr);
    merge_sort::parallel_mergesort(&arr);
    pdqsort::parallel_pdqsort(&arr);
    pdqsort::serial_pdqsort(&arr);
}