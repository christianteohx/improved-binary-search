
use rand::random;

const DEFAULT_ARRAY_SIZE:usize = 100000;

pub fn create_array() -> Vec<u32> {

  let mut arr = Vec::new();

  let mut num: u32 = 0;

  for _ in 0..DEFAULT_ARRAY_SIZE {

    num += (random::<u32>() % 100) + 1;
    arr.push(num);
  }

  arr
  
}

pub fn improved_binary_search(num:u32, arr: &Vec<u32>) -> u32 {

    let mut iteration: u32 = 0;
    let mut low = 0;
    let mut high = (arr.len() as u32) - 1;

    while arr[low as usize] < num && num < arr[high as usize] {

        iteration += 1;

        let low_diff = num - arr[low as usize];
        let high_diff = arr[high as usize] - num;

        let prev_high = high;
        let prev_low = low;

        // println!("low_diff: {}, high_diff: {}", low_diff, high_diff);

        if low_diff < high_diff {
            high = low + low_diff + 1;
        } else {
            low = high - high_diff - 1;
        }

        // println!("low: {}, high: {}", low, high);

        if high - low == 0 {
            return iteration
        }

        if (prev_high-prev_low) / (high-low) < 2 {
            break;
        }

        // println!("new arr: {:?}", &arr[low as usize..high as usize].to_vec());

    }

    // println!("Improved local iterations: {}", iteration);

    iteration + binary_search(num, &arr[low as usize..high as usize].to_vec(), true)

}

pub fn binary_search(num:u32, arr: &Vec<u32>, from_improved: bool) -> u32{

    // println!("arr: {:?}", arr);

    let mut low = 0;
    let mut high = (arr.len() as u32) - 1;
    let mut iteration = 0;

    while low <= high{

        iteration += 1;

        let mid = (low + high) / 2;

        if arr[mid as usize] == num {
            return iteration
        } else if arr[mid as usize] < num {
            if mid < arr.len() as u32 - 1{
                low = mid + 1;
            } else {
                break;
            }
        } else {
            if mid > 0 {
                high = mid - 1;
            } else {
                break;
            }
        }

        if low > high {
            break;
        }

    }

    if from_improved {
        println!("Improved used {} iterations", iteration);
    } else {
        println!("Binary used {} iterations", iteration);
    }

    iteration

}