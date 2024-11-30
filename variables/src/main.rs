fn main() {
  mutate_variable();
  constant();
  shadowing();
}

fn mutate_variable() {
  println!("Demonstrating mutable variable");

  let mut x = 5;
  println!("The value of x is: {x}");
  
  x = 6;
  println!("The value of x is: {x}");
  println!();
}

fn constant() {
  println!("Demonstrating constant");

  // https://doc.rust-lang.org/stable/reference/const_eval.html
  const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
  println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");
  println!();
}

fn shadowing() {
  println!("Demonstrating shadowing");

  // shadowing is different then mutating
  // we can perform transformations on the variable
  // but have the variable be immutable after 
  // the transformations are complete
  let x = 5;

  let x = x + 1;

  {
    let x = x * 2;
    println!("The value of x in the inner scope is: {x}");
  }
  
  // no longer in the inner scope
  // so x * 2 is no longer shadowing x + 1
  println!("The value of x is: {x}");
  println!();
}