use std::cmp;
// Say hi
pub fn greeting(string: &str){
    println!("{}", string);
}

// reverse a string !
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

// remove a certain character from a string
pub fn remove(string: &str, character: &str) {
    println!("{:?}", string.replace(character, ""));
}

// sum all numbers of an array
pub fn sum(array: [i32; 10]) {
    let mut sum = 0;
    for n in 0..array.len() {
        sum += array[n];
    }
    println!("The sum of the values of the array is {}", sum);
}

// chunk vectors
pub fn chunk(vector: Vec<usize>, number: usize) {
    let number_of_chunks: usize;
    if vector.len() % number == 0 {
        number_of_chunks = vector.len() / number;
    } else {
        number_of_chunks = vector.len() / number + 1;
    }
    println!("{}", number_of_chunks);
    let mut count = 0;
    for x in 0..number_of_chunks {
    // create a new vector with the "variable number" length
        let mut new_vec: Vec<usize> = Vec::new();
        while new_vec.len() < number {
            if count > vector.len() - 1 {
            break;
        }
        new_vec.push(vector[count]);
        count += 1;
    }
    println!("{:?}", new_vec);
  }

  //count occurences
  //pub fn occurencies(array: [char ; ]) {

  //}
}
