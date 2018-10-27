fn main() {
  another_function();
  func_with_params(15);

  println!("the value of x is: {}", five());
}

fn another_function() {
  println!("Another function.");
}

fn func_with_params(x: i32) {
  println!("the value of x is: {}", x);
}

fn five() -> i32 {
  5
}