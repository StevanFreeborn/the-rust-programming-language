fn main() {
  immutable_reference();
}

fn immutable_reference() {
  let s1 = String::from("hello");

  // the action of creating a reference like this
  // is called borrowing
  // references are immutable by default
  // you cam modify something you are borrowing
  let len = calculate_length(&s1);

  println!("The length of '{s1}' is {len}.");

  // use a reference to s and not take
  // ownership of s
  fn calculate_length(s: &String) -> usize {
    s.len()
  }
}

fn mutable_reference() {
  let mut s = String::from("hello");

  // you can only have one mutable reference
  // allows for mutation, but in a controlled fashion
  // prevents data races
  change(&mut s);

  fn change(some_string: &mut String) {
    some_string.push_str(", world");
  }
}
