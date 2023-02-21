mod merge_sort;


fn main(){
    merge_sort::serial_mergesort_test();
    merge_sort::parallel_mergesort_test();
}