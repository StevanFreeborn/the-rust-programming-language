fn main() {
  without_slices();
  string_slices();
}

fn without_slices() {

  let mut s = String::from("hello world");

  let _word = first_word(&s); // word will get the value 5

  s.clear(); // this empties the String, making it equal to ""

  // word still has the value 5 here, but there's no more string that
  // we could meaningfully use the value 5 with. word is now totally invalid!

  fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
  
    for (i, &item) in bytes.iter().enumerate() {
      if item == b' ' {
        return i;
      }
    }
  
    s.len()
  }
}

fn string_slices() {
  let mut s = String::from("hello world");

  let _word = first_word(&s);

  s.clear();

  fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
}