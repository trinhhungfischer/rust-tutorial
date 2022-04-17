// Array - Fixed list where element are the same data types

pub fn run() {
  let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

  // Re-assign value
  numbers[2] = 20;

  println!("{:?}", numbers);
  
  // Add on to vector
  numbers.push(2);
  numbers.push(3);

  // Get single value
  println!("Single value: {}", numbers[0]);
  
  // Get array length
  println!("Vector Length: {}", numbers.len());

  // Vector are stack alocated
  println!("Array Occupies {} bytes", std::mem::size_of_val(&numbers));

  // Get slice of vector
  let slice: &[i32] = &numbers[0..2];
  println!("Slice Vector: {:?}", slice);

  // Loop through vector values
  for x in numbers.iter() {
    println!("Number: {}", x);
  }

  // Lloop & mutate value
  for x in numbers.iter_mut(){
    *x += 2;
  }
  println!("Number Vec after Mutate: {:?}", numbers);
}