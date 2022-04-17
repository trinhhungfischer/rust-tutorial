// Reference Pointer - Pointer to a resourse in memory
pub fn run() {
  // Primitive array
  let arr1 = [1, 2, 3];
  let arr2 = arr1;

  // With non-primitives, if you assign another variable to a piece
  // of data, the first variable will no longer hold that value

  // Vector
  let vec1 = vec![1, 2, 3];
  let vec2 = &vec1;

  println!("Values: {:?}", (&vec1, &vec2));
}