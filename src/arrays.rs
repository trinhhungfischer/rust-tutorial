// Array - Fixed list where element are the same data types

pub fn run() {
  let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

  // Re-assign value
  numbers[2] = 20;

  println!("{:?}", numbers);
  
  // Get single value
  println!("Single value: {}", numbers[0]);
  
  // Get array length
  println!("Array Length: {}", numbers.len());

  // Array are stack locate
  println!("Array Occupies {} bytes", std::mem::size_of_val(&numbers));

  // Get slice
  let slice: &[i32] = &numbers[0..2];
  println!("Slice: {:?}", slice);
}