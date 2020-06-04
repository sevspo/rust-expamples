// arrays in rust are a list of items of  the same type
// the size of the arrays is also fixed, you have to asign it.
// the compiler is also going to infer the type and size if it can.

use std::mem;

pub fn run() {
  let mut arr: [i32; 6] = [1, 2, 3, 4, 5, 6];

  //reassign a value (because you cannot ad on to it!)
  arr[5] = 20;

  // print using debug mode
  println!("debug mode {:?}", arr);

  //get single value
  println!("single value{}", arr[0]);

  // get lentght
  println!("Length: {}", arr.len());

  // arrays are stack allocated. here, the ampersant says that we want to pass a ref.
  println!("Size in bytes: {}", mem::size_of_val(&arr));

  // get a slice of the array
  let slice: &[i32] = &arr[0..2];
  println!("the slice: {:?}", slice);
}
