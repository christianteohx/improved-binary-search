
use rand::random;

const DEFAULT_ARRAY_SIZE:usize = 1000000;

fn create_array() -> Vec<u32> {

  let mut arr = Vec::new();

  for _ in 0..DEFAULT_ARRAY_SIZE {

    let num = random::<u32>() % DEFAULT_ARRAY_SIZE as u32;
    arr.push(num);
  }

  arr
  
}

fn improved_binary_search() -> i32 {

    let mut arr = [];
    let target = 5;
    let mut left = 0;
    let mut right = arr.len() as i32 - 1;
    while left <= right {
        let mid = left + (right - left) / 2;
        if arr[mid as usize] == target {
            return mid;
        } else if arr[mid as usize] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    -1
}