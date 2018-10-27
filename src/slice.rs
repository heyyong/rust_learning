fn main() {
  let mut s = String::from("hello world");

  let word = first_word(&s);

  // mutable borrow here throw an error
  // s.clear();

  println!("{}", word);
}

fn first_word(s: &str) -> &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[..i];
    }
  }

  &s[..]
}