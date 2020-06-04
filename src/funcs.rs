pub fn run() {
  greeting("Hello", "Sevi");

  // Bind function value to variable
  let get_sum = add(5, 15);
  println!("{}", get_sum);

  // Closure ???
  let add_nums = |x1: i32, x2: i32| x1 + x2;
  println!("{}", add_nums(5, 2));
}

fn greeting(greet: &str, name: &str) {
  println!("Hello {} and I am {}", greet, name);
}

// if we don't use a semicolon it means we want to return this expression
fn add(n1: i32, n2: i32) -> i32 {
  n1 + n2
}
