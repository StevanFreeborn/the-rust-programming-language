fn main() {
  parse_number();
  floating_point();
  calculate();
  boolean();
  characters();
  tuples();
  arrays();
}

fn parse_number() {
  let parsed_number: u32 = "42".parse().expect("Not a number!");
  println!("parsed number: {parsed_number}");
}

fn floating_point() {
  let x = 2.0;
  let y: f32 = 3.0;

  println!("x: {x}, y: {y}");
}

fn calculate() {
  // addition
  let sum = 5 + 10;

  // subtraction
    let difference = 95.5 - 4.3;

  // multiplication
  let product = 4 * 30;

  // division
  let quotient = 56.7 / 32.2;
  let truncated = -5 / 3; // Results in -1

  // remainder
  let remainder = 43 % 5;

  println!("sum: {sum}");
  println!("difference: {difference}");
  println!("product: {product}");
  println!("quotient: {quotient}");
  println!("truncated: {truncated}");
  println!("remainder: {remainder}");
}

fn boolean() {
  let t = true;
  let f: bool = false;

  println!("t: {t}, f: {f}");
}

// char literals use single quotes
// string literals use double quotes
fn characters() {
  let c = 'z';
  let z = 'ℤ';
  let heart_eyed_cat = '😻';

  println!("c: {c}, z: {z}, heart_eyed_cat: {heart_eyed_cat}");
}

fn tuples() {
  let tup: (i32, f64, u8) = (500, 6.4, 1);

  // destructuring
  // useful for pattern matching
  let (x, y, z) = tup;

  println!("x: {x}, y: {y}, z: {z}");

  // access tuple elements by index
  let five_hundred = tup.0;
  let six_point_four = tup.1;
  let one = tup.2;

  // this is known as a "unit"
  // represents an empty tuple
  // let empty = ();

  println!("five_hundred: {five_hundred}, six_point_four: {six_point_four}, one: {one}");
}

fn arrays() {
  // arrays have fixed length
  // vectors are more flexible
  let _a = [1, 2, 3, 4, 5];

  // arrays are stack allocated
  // vectors are heap allocated
  let _a: [i32; 5] = [1, 2, 3, 4, 5];

  // initialize array with same value
  let a = [3; 5]; // [3, 3, 3, 3, 3]

  // access array elements by index
  let first = a[0];
  let second = a[1];

  println!("first: {first}, second: {second}");
}