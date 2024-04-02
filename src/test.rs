
use crate::algorithms;
use std::time::Instant;

#[test]
fn test_iterations() -> Result<(), Box<dyn std::error::Error>> {

  let mut arr: Vec<u32> = vec![];

  for i in 0..100 {
    arr.push(i);
  }

  let num = 14;

  let improved_results = algorithms::improved_binary_search(num, &arr);
  println!("Improved iterations: {}", improved_results);

  let results = algorithms::binary_search(num, &arr, false);
  println!("Iterations: {}", results);

  Ok(())
}

#[test]
fn test_improved_binary_search() -> Result<(), Box<dyn std::error::Error>> {

  let test_count = 1000;
  let search_count = 1000;

  let mut improved_time_list: Vec<f32> = Vec::new();
  let mut improved_iterations: Vec<u32> = Vec::new();
  let mut time_list: Vec<f32> = Vec::new();
  let mut iterations: Vec<u32> = Vec::new();

  for _ in 0..test_count {

    let arr = algorithms::create_array();
    let len = arr.len();
    
    let min = arr[0];
    let max = arr[len-1];

    for i in 0..search_count {

      let mut num = rand::random::<u32>() % len as u32;
      
      if num < min {
        num *= min;
      } 

      if num > max {
        num %= max;
      }

      num = min + 10;

      let improved_start = Instant::now();

      let improved_result = algorithms::improved_binary_search(num, &arr);

      let improved_end = Instant::now();
      let improved_time = (improved_end-improved_start).as_secs_f32();
      improved_time_list.push(improved_time);
      improved_iterations.push(improved_result);

      let start = Instant::now();
      let results = algorithms::binary_search(num, &arr, false);
      let end = Instant::now();
      let time = (end-start).as_secs_f32();
      time_list.push(time);
      iterations.push(results);

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

  Ok(())
}