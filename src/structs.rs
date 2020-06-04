// used to create custom data types, they are similar to classes

// traditional struct
struct Color {
  red: u8,
  green: u8,
  blue: u8,
}

// tuple struct
struct TupleStruct(u8, u8, u8);

// complex example
// I think a struct is more like an interface as it is a class?
struct Person {
  first_name: String,
  last_name: String,
}

impl Person {
  fn new(first: &str, last: &str) -> Person {
    Person {
      first_name: first.to_string(),
      last_name: last.to_string(),
    }
  }

  fn full_name(&self) -> String {
    format!("{} {}", self.first_name, self.last_name)
  }

  fn set_last_name(&mut self, last: &str) {
    self.last_name = last.to_string();
  }
}

pub fn run() {
  let mut c = Color {
    red: 255,
    green: 0,
    blue: 0,
  };

  c.green = 200;

  println!("Color: {} {} {}", c.red, c.green, c.blue);

  let mut tup_struct = TupleStruct(255, 0, 0);

  tup_struct.2 = 50;

  println!(
    "Tup Color: {} {} {}",
    tup_struct.0, tup_struct.1, tup_struct.2
  );

  let mut p = Person::new("Jane", "Doe");

  println!("Person is {} {}", p.first_name, p.last_name);

  println!("Full Name: {}", p.full_name());

  p.set_last_name("Hunspi");
  println!("Person is {} {}", p.first_name, p.last_name);
}
