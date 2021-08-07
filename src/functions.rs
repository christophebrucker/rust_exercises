use std::cmp;

// reverse a string
pub fn reverse(string: &str) {
 println!("{}", string.chars().rev().collect::<String>());
}

// find the max of two numbers
pub fn max(n1: i32, n2: i32) {
  println!("The max betwween {} and {} is {}", n1, n2,cmp::max(n1, n2));
}

// sort array by value
pub fn sort(array: [i32; 4]) {
  let mut array_copy = array;
  array_copy.sort();
  println!("The array sorted is {:?}", array_copy);
}
