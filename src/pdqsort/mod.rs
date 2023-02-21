use rayon::prelude::*;
use rand::Rng;

pub fn parallel_rayon_test(array_size: usize){
    let mut arr: Vec<i32> = (0..array_size).map(|_| rand::thread_rng().gen::<i32>()).collect();
    let now = std::time::Instant::now();
    arr.par_sort_unstable();
    println!("Parallel rayon time: {}ms", now.elapsed().as_millis());
}