use std::time::Instant;

mod algorithms;
#[cfg(test)] mod test;

fn main() {
    
    // let arr = algorithms::create_array();
    let arr: Vec<u32> = vec![31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 43, 44, 47, 49, 50];
    let num = 44;

    let mut time_list: Vec<f32> = Vec::new();
    let start = Instant::now();
    
    let results = algorithms::improved_binary_search(num, &arr);

    let end = Instant::now();
    let time = (end-start).as_secs_f32();
    time_list.push(time);
    println!("Found {} at index {}", num, results);
      

}
