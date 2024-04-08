
use crate::algorithms;
use std::time::Instant;

#[test]
fn test_iterations() -> Result<(), Box<dyn std::error::Error>> {

  let mut arr: Vec<u32> = vec![];

  for i in 0..1000000 {
    arr.push(i*68);
  }

  let num = 14;

  let improved_results = algorithms::improved_binary_search(num, &arr);
  // println!("Improved iterations: {}", improved_results);

  let results = algorithms::binary_search(num, &arr, false);
  // println!("Iterations: {}", results);

  Ok(())
}

#[test]
fn test_improved_binary_search() -> Result<(), Box<dyn std::error::Error>> {

  let test_count = 10;
  let search_count = 150;

  let mut improved_time_list: Vec<f32> = Vec::new();
  let mut improved_iterations: Vec<u32> = Vec::new();
  let mut time_list: Vec<f32> = Vec::new();
  let mut iterations: Vec<u32> = Vec::new();
  let mut iteration_count = 0;
  let mut search_list: Vec<u32> = Vec::new();

  for test_num in 0..test_count {

    println!("Test {}/{}", test_num+1, test_count);

    // let arr = algorithms::create_random_array(100000000);
    let arr = algorithms::create_step_array(100000000, 3);
    let len = arr.len();
    
    let min = arr[0];
    let max = arr[len-1];

    for search_num in 0..search_count {

      println!("Search {}/{}", search_num+1, search_count);

      let mut num = rand::random::<u32>() % max as u32;
      search_list.push(num);
      
      if num < min {
        num *= min;
      } 

      if num > max {
        num %= max;
      }

      // num = min + 10;

      let improved_start = Instant::now();

      let improved_result = algorithms::improved_binary_search(num, &arr);

      let improved_end = Instant::now();
      let improved_time = (improved_end-improved_start).as_secs_f32();
      improved_time_list.push(improved_time);
      improved_iterations.push(improved_result);

      let high = (arr.len() as u32) - 1;

      let start = Instant::now();
      let results = algorithms::binary_search(num, &arr, false, 0, high);
      let end = Instant::now();
      let time = (end-start).as_secs_f32();
      time_list.push(time);
      iterations.push(results);

      if improved_result > results {
        iteration_count += 1;
      } else if improved_result < results {
        iteration_count -= 1;
      }

    }
  }

  let improved_total_time: f32 = improved_time_list.iter().sum();
  let improved_average_time: f32 = improved_total_time / (improved_time_list.len() as f32);
  let improved_total_iterations: u32 = improved_iterations.iter().sum();
  let improved_average_iterations: f32 = improved_total_iterations as f32 / (improved_iterations.len() as f32);
  println!("Improved average time: {}", improved_average_time);
  println!("Improved average iterations: {}", improved_average_iterations);

  let total_time: f32 = time_list.iter().sum();
  let average_time: f32 = total_time / (time_list.len() as f32);
  let total_iterations: u32 = iterations.iter().sum();
  let average_iterations: f32 = total_iterations as f32 / (iterations.len() as f32);
  println!("Average time: {}", average_time);
  println!("Average iterations: {}", average_iterations);

  if iteration_count < 0 {
    println!("Improved binary search is better");
  } else if iteration_count > 0 {
    println!("Binary search is better");
  } else {
    println!("Both are equal");
  }

  println!("Iteration count: {}", iteration_count);
  println!("Search list: {:?}", search_list);

  Ok(())
}