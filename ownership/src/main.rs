fn main() {
  println!("Hello, world!");
  scope_example();
  strings_and_ownership();
  variables_and_data_with_move();
  cloning();
  copy_trait();
  ownership_and_functions();
  return_values();
  ceremony_for_dealing_with_ownership_and_scope();
}

fn scope_example() {
  {                  // s is not valid here, it’s not yet declared
    let _s = "hello"; // s is valid from this point forward
                     // do stuff with s
  }                  // this scope is now over, and s is no longer valid
}

fn strings_and_ownership() {
  let mut s = String::from("hello");
  s.push_str(", world!");
  println!("{s}");

  // Rust automatically calls drop function
  // at the closing curly bracket...similar
  // to R.A.I.I. patterns in C++
  {
    let _s = String::from("hello"); // s is valid from this point forward
                                            // do stuff with s
  }                                         // this scope is now over, and s is no
                                            // longer valid
}

fn variables_and_data_with_move() {
  let x = 5;
  let _y = x;

  // a String has three parts
  // - pointer to memory on heap that holds content
  // - length
  // - capacity
  // these are stored on the stack
  // the heap holds the actual content of the string
  // in this example we copy the data on the stack
  // not the content on the heap
  let s1 = String::from("hello");
  let _s2 = s1;

  // this won't compile because binding s1
  // to s2 results in trying to borrow a moved
  // value
  // in other languages its common for this action
  // to result in a shallow copy...but here it's a
  // literal move.
  // in addition it means Rust will never
  // make deep copies of data by default
  // println!("{s1}, world!");
}

fn cloning() {
  let s1 = String::from("hello");
  let s2 = s1.clone();

  // now we've actually allocated memory to the stack and heap
  // twice for the contents "hello"
  println!("s1 = {s1}, s2 = {s2}");
}

// Rust supports placing a special annotation
// called the Copy trait on types that
// are stored exclusively on the stack
// These types are therefore copied instead
// of moved
// Copy trait is mutually exclusive with
// the Drop trait
fn copy_trait() {
  let x = 5;
  let y = x;

  println!("x = {x}, y = {y}");
}

fn ownership_and_functions() {
  let s = String::from("hello"); // s comes into scope

  takes_ownership(s); // s's value moves into the function...
  // ... and so is no longer valid here

  // can't do this...trying to borrow moved value
  // println!("{s}");

  let x = 5; // x comes into scope

  makes_copy(x); // x would move into the function,
  // but i32 is Copy, so it's okay to still
  // use x afterward

  // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

  fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
  } // Here, some_string goes out of scope and `drop` is called. The backing
    // memory is freed.

  fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
  } // Here, some_integer goes out of scope. Nothing special happens.
}

fn return_values() {
  let _s1 = gives_ownership(); // gives_ownership moves its return
  // value into s1

  let s2 = String::from("hello"); // s2 comes into scope

  let _s3 = takes_and_gives_back(s2); // s2 is moved into
  // takes_and_gives_back, which also
  // moves its return value into s3

  fn gives_ownership() -> String { // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
    // moves out to the calling
    // function
  }

  // This function takes a String and returns one
  fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
    // scope
    a_string  // a_string is returned and moves out to the calling function
  }
}

fn ceremony_for_dealing_with_ownership_and_scope() {
  let s1 = String::from("hello");

  let (s2, len) = calculate_length(s1);

  println!("The length of '{s2}' is {len}.");

  fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
  }
}