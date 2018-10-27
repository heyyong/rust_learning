fn main() {
  let number = 6;

  if number % 4 == 0 {
    println!("condition was divisible by 4");
  } else if number % 3 == 0 {
    println!("number was divisible by 3");
  } else {
    println!("not condition");
  }

  let condition = true;

  let number = if condition {
    5
  } else {
    6
  };

  println!("number is {}", number);

  let number = {
    19
  };

  println!("number is {}", number);
}