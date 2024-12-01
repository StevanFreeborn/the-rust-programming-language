fn main() {
  println!("Hello, world!");
  let result = add(1, 2);
  println!("1 + 2 = {}", result);
  expression();
  let val = return_value();
  println!("The value of val is: {}", val);
  let early_val = return_early();
  println!("The value of early val is: {}", early_val);
}

// must declare parameter types
// must declare return type
fn add(a: i32, b: i32) -> i32 {
  a + b
}

fn expression() {
  let _x = 5;
  let y = {
    let x = 3;
    x + 1 // no semicolon
  };
  println!("The value of y is: {}", y);
}

// implicit return of last expression
fn return_value() -> i32 {
  1 + 1 // no semicolon
}

fn return_early() -> i32 {
  if true {
    return 1;
  }
  1 + 1
}