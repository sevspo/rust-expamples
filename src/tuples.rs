// tuples group together elements of different types
// you can have a  maximum of 12 elements

pub fn run() {
  let sevi: (&str, &str, i8) = ("sevi", "age", 38);

  println!("my name is {} and my {} is {}", sevi.0, sevi.1, sevi.2);
}
