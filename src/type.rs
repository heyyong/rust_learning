fn main() {
  let guess: u32 = "42".parse().expect("Not a number!");
  println!("guess is {}", guess);
  let number8: u8 = 255;
  println!("{}", number8);

  let normal = 2147483647;
  println!("{}", normal);

  let force: f32 = 16.5 / 5.0;
  println!("{}", force);

  let _t = true;
  let _f:bool = true;

  // char
  let _char = 'a';
  // string
  let _str = "asdf";

  // tuple
  let tup : (i32, f64, u8) = (500, 6.4, 12);
  let (a, _b, _c) = tup;
  println!("{}", a);
  println!("{}", tup.1);
}