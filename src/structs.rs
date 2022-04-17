// Structs - Used to create custom data types

// Traditional Struct
struct Color {
  red: u8,
  green: u8,
  blue: u8
}

// Tuple Struc
struct Color2(u8, u8, u8);

struct Person {
  first_name : String, 
  last_name: String
}

impl Person {
  // Construct person
  fn new(first: &str, last: &str) -> Person {
    Person {
      first_name: first.to_string(),
      last_name: last.to_string(),
    }
  }

  // Get Full Name
  fn full_name(&self) -> String {
    format!("{} {}", self.first_name, self.last_name)
  }

  // Set last name
  fn set_last_name(&mut self, last: &str) {
    self.last_name = last.to_string();
  }

  // Name to tuple
  fn to_tuple(self) -> (String, String) {
    (self.first_name, self.last_name)
  }
}


pub fn run() {
  let mut c  = Color {
    red: 255,
    green: 0,
    blue: 0
  };

  c.red = 200;
  
  println!("Color: {} {} {}", c.red, c.blue, c.green);

  let c2 = Color2(1, 2, 3);

  println!("Color: {} {} {}", c2.0, c2.1, c2.2);
 
  let mut p = Person::new("Hung", "Trinh");

  println!("Person {} {}", p.first_name, p.last_name);
  println!("Person {}", p.full_name());

  p.set_last_name("Le");
  println!("Person {}", p.full_name());

  println!("Person {:?}", p.to_tuple());

}