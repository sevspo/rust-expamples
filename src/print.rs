pub fn run() {
  // print to console
  println!("Hello Severin");

  // basic fromatting
  println!("{} is from {}", "Sevi", "Ostermundigen");

  // postitional arguments
  println!(
    "{0} is form {1} and {0} likes {2}",
    "Sevi", "Bolligen", "Sara"
  );

  // named arguments
  println!(
    "{name} likes to play {activity}",
    name = "Sevi",
    activity = "Soccer"
  );

  // placeholder traits
  println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

  // placeholder for debug traits
  println!("{:?}", (12, true, "hello"));

  // basic math
  println!("10 + 10 = {}", 10 + 5);
}
