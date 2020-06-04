pub fn run() {
  let mut count = 0;
  let mut count_1 = 0;

  // infinite loop
  loop {
    count += 1;
    println!("{}", count);

    if count == 8 {
      break;
    }
  }

  //while loops
  while count_1 <= 100 {
    if count_1 % 15 == 0 {
      println!("fizzbuzz");
    } else if count_1 % 3 == 0 {
      println!("fizz");
    } else if count_1 % 5 == 0 {
      println!("buzz")
    } else {
      println!("{}", count_1);
    }
    count_1 += 1;
  }

  println!("###################### again: ######################");

  // for range
  for x in 0..100 {
    if x % 15 == 0 {
      println!("fizzbuzz");
    } else if x % 3 == 0 {
      println!("fizz");
    } else if x % 5 == 0 {
      println!("buzz")
    } else {
      println!("{}", x);
    }
  }
}
