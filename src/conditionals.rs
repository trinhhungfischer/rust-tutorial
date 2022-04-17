pub fn run() {
  greeting("Good Morning", "Trinh Hung");
  
  // Call function and pass argument
  let get_sum :i32 = add(32, 32);
  println!("Get sum: {}", get_sum);

  // Closure
  let n3:i32 = 10;
  let add_num = |n1: i32, n2: i32| n1 + n2 + n3;
  println!("C Sum: {}", add_num(3, 4));

}

fn greeting(greet: &str, name: &str) {
  println!("{} {}, nice to meet you!", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
  n1 + n2
}