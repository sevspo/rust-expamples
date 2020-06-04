// Primitve &str = Immutable primitive fixed length string somewhere in memory
// String = Growable, heap allocated data structure - use when you need to modify or own string data.

pub fn the_strings() {
  let primitive_str = "hello world";
  let mut growable_str = String::from("hello new world");

  println!("{} and {}", primitive_str, growable_str);

  // get length, works for both
  println!("Length: {}", primitive_str.len());

  // push character
  growable_str.push('w');

  println!("{}", growable_str);

  // capacity in bytes
  println!("Capacity: {}", growable_str.capacity());

  // check if emty

  // contains substring

  // repalace

  // loop over
  for word in growable_str.split_whitespace() {
    println!("{}", word);
  }

  let mut s = String::with_capacity(10);
  s.push('b');
  s.push('w');

  // if an assertion passes, it does nothing, only if it fails the compiler will tell you
  assert_eq!(s.len(), 3);
  println!("{}", s);
}
