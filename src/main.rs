mod merge_sort;
mod pdqsort;
const ARRAY_SIZE: usize = 1300000;


fn main(){
    merge_sort::serial_mergesort_test(ARRAY_SIZE);
    merge_sort::parallel_mergesort_test(ARRAY_SIZE);
    pdqsort::parallel_rayon_test(ARRAY_SIZE);
}