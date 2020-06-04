// this is as in JS
// but there is no terary
// but there is a shorthand

pub fn run() {
  let age: u8 = 18;
  let check_id: bool = false;

  if age >= 21 && check_id {
    println!("Bartenter: What do you want to drink?")
  } else if age < 21 {
    println!("you have to leave")
  }

  //shorthand
  let is_of_age = if age >= 21 { true } else { false };
  println!("{}", is_of_age)
}
