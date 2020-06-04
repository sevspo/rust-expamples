// vectors are expandable arays and heap allocated
//

use std::mem;

pub fn run() {
  let mut vec: Vec<i32> = vec![1, 2, 3, 4, 5, 6];

  // reassign
  vec[5] = 20;

  // push to vector
  vec.push(33);
  vec.push(34);

  // print using debug mode
  println!("debug mode {:?}", vec);

  //get single value
  println!("single value{}", vec[0]);

  // get lentght
  println!("Length: {}", vec.len());

  // here, the ampersant says that we want to pass a ref.
  println!("Size in bytes: {}", mem::size_of_val(&vec));

  // get a slice of the vec
  let slice: &[i32] = &vec[0..2];
  println!("the slice: {:?}", slice);

  //loop over vec and
  for number in vec.iter() {
    println!("{}", number);
  }

  // mutate array
  for number in vec.iter_mut() {
    *number *= 2;
  }

  println!("{:?}", vec)
}
