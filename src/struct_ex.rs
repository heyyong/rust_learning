
#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32
}

fn main() {
  let width1 = 30;
  let height1 = 50;

  println!(
    "The area of the rectangle is {} square pixels.",
    area(width1, height1)
  );

  ///
  let rect1 = (30, 50);
  println!(
    "The area of the rectangle is {} square pixels.",
    tuple_area(rect1)
  );

  let rect2 = Rectangle { width: 30, height: 50 };

  println!(
    "The area of the rectangle is {} square pixels.",
    struct_area(&rect2)
  );

  println!(
    "rect2 is {:#?}",
    rect2
  );
}

fn struct_area(rectangle: &Rectangle) -> u32 {
  rectangle.width * rectangle.height
}

fn tuple_area(dimensions: (u32, u32)) -> u32 {
  dimensions.0 * dimensions.1
}

fn area(width: u32, height: u32) -> u32 {
  width * height
}