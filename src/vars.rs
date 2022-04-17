

pub fn run(){
  let name = "Trinh Hung";
  let mut age = 37;

  age = 38;
  println!("My name is {} and I am {}", name, age);

  // Define constant
  const ID: i32 = 001;
  println!("ID: {}", ID);

  // Assigin multiple vars
  let (my_name, my_age) = ("Trinh Hung", 18);
  
  println!("My name is {} and I am {}", my_name, my_age);

}