// we are using a "crate" called rand
// cargo doc --open for more info
use rand::Rng;

use std::cmp::Ordering;

// bring the io library into scope
// https://doc.rust-lang.org/stable/std/prelude/index.html
use std::io;

fn main() {
  println!("Guess the number!");

  // remember secret_number is immutable
  // rng that is local to the current thread of exe
  // range expression start..=end is inclusive of both bounds
  let secret_number = rand::thread_rng().gen_range(1..=100);

  loop {
    println!("Please input your guess.");

    // variables in Rust are immutable by default
    // i.e. let apples = 5; is immutable
    // to make a variable mutable, use the mut keyword
    // i.e. let mut bananas = 5;

    // :: indicates that new is an associated function of the String type
    // associated functions are implemented on types, rather than on instances of types
    // associated functions are like static methods in C#
    let mut guess = String::new();

    // & indicates that this argument is a reference
    // references are immutable by default
    // to make a reference mutable, use the mut keyword
    io::stdin()
      .read_line(&mut guess)
      .expect("Failed to read line");

    // Rust allows shadowing
    // Shadowing allows us to reuse the guess variable name
    // Shadowing often used when converting a variable from one type to another
    // Gets rid of the need to create a new variable name
    // i.e. guessAsString -> guess
    // : is a type annotation
    // dis match with the result type is noice 👏🏻
    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => {
        println!("Error: Please enter a number.");
        continue;
      },
    };

    println!("You guessed: {}", guess);

    // passing by reference
    // match is similar to switch in C#
    // Rust has a strong, static type system, but does have type inference
    // numbers default to i32
    match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too small!"),
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal => {
        println!("You win!");
        break;
      },
    }
  }
}
