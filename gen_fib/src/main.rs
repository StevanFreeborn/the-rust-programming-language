use std::io;

fn get_input() -> u32 {
  let mut input = String::new();

  io::stdin()
    .read_line(&mut input)
    .expect("FATAL: Failed to read line");

  let input: u32 = match input.trim().parse() {
    Ok(num) => num,
    Err(_) => {
      println!("ERROR: Please enter a number");
      return get_input();
    },
  };

  input
}

// a fibonacci number is the sum of the two preceding ones, starting from 0 and 1
// 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, ...
fn generate_fibonacci(n: u32) -> u32 {
  if n == 0 {
    return 0;
  }
  
  if n == 1 {
    return 1;
  }

  let mut first_number = 0;
  let mut second_number = 1;
  let mut result = 0;

  for _ in 2..=n {
    result = first_number + second_number;
    first_number = second_number;
    second_number = result;
  }

  result
}

fn main() {
  println!("Generate nth Fibonacci number");

  println!("Enter the value of n: ");
  let n = get_input();

  let result = generate_fibonacci(n);

  println!("The {}th Fibonacci number is {result}", n);
}
