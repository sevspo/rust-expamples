// variables hold primitives or refs
// they are immutable by default
// they are block scoped

pub fn vars() {
  // basic assignment
  let name = "Severin Spoerri";
  let mut age = 38;

  println!("Hello {} and I am {}", name, age);
  age = 39;
  println!("Hello {} and I am {}", name, age);

  // define a constant
  const ID: i32 = 001;
  println!("ID: {}", ID);

  // bulk assignment
  let (my_name, my_age) = ("Sevi", 37);
  println!("Hello {} and I am {}", my_name, my_age);
}
