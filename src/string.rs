pub fn run() {
  let mut hello = String::from("Hello ");
  
  // Get length
  println!("Length: {}", hello.len());

  // Add a char in back string
  hello.push('W');

  // Add a string
  hello.push_str("orld!");

  // Capacity in byes
  println!("Capacity: {}", hello.capacity());

  // Check if empty
  println!("Is empty: {}", hello.is_empty());

  // Contains
  println!("Contain 'World': {}", hello.contains("World"));

  // Replace
  println!("Replace: {}", hello.replace("World", "There"));

  // Loop through string by whitespace
  for word in hello.split_whitespace() {
    println!("{}", word);
  }

  // Create string with capacity
  let mut s =String::with_capacity(10);
  s.push('a');
  s.push('a');

  // Assert testing
  assert_eq!(2, s.len());
  assert_eq!(11, s.len());
  

  println!("{}", hello);
}