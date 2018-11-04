struct Point<T> {
  x: T,
  y: T
}

impl<T> Point<T> {
  fn x(&self) -> &T {
    &self.x
  }
}


fn main() {
  let integer = Point {
    x: 5,
    y: 10
  }

  let float = Point {
    x: 1.0,
    y: 4.0
  }

  println!("p.x = {}", p.x())

  let number_list = vec![34, 50, 25, 100, 65];

  println!("The largest number is {}", find_largest(&number_list));
}

fn find_largest<T>(list: &[T]) -> T {
  let mut largest = list[0];

  for &number in list.iter() {
    if number > largest {
      largest = number;
    }
  }

  largest
}

