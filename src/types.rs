/*
Primitve Types

Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128
i32 is default, or infered most of the time if not specified
u = unsigned integer, i = signed integer.

Floats: f32, f64
f64 is default, or infered

Boolean: (bool)
Characters (char)
Tuples
Arrays
*/

// rust is a statically typed language and it must know the types at compile time, but the compiler can usually infer the type based on the value and usage.

pub fn the_types() {
  // is i32
  let x = 1;

  // is f64
  let y = 2.5;

  // add explicit type
  let z: i64 = 454545454545445;

  // find max size
  println!("max size i32: {}", std::i32::MAX);

  // bool
  let is_active: bool = true;
  let is_grater: bool = 10 > 9;
  // unicode Characters (have to be in single quotes!)
  let a1 = 'a';
  // and this works becuase we still have a single character
  let face = '\u{1F600}';

  // and by the way, there is no hoisting
  println!("{:?}", (x, y, z, is_active, is_grater, a1, face));
}
