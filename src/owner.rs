fn main() {
  let mut s = String::from("hello");

  s.push_str(", world!");

  println!("{}", s);

  let s1 = String::from("hello");
  let s2 = s1;

  // s1 moved here
  println!("{}, world", s2);

  let s3 = s2.clone();

  println!("{}", s3);

  let s5 = "hello";
  let s6 = s5;
  println!("{}", s5);

  let str = String::from("hello");

  takes_ownership(str);

  let s2 = String::from("hello");

  let s3 = take_and_give_back(s2);

  println!("{}", s3);
}

fn take_and_give_back(a_string: String) -> String {
  a_string
}

fn gives_ownership() -> String {
  let some_string = String::from("hello");

  some_string
}

fn takes_ownership(something: String) {
  println!("{}", something);
}