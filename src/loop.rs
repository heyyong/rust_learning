fn main() {
  let mut number = 0;
  loop {
    if number > 5000 {
      break;
    }
    println!("again: {}", number);
    number += 1;
  }

  let mut number = 3;

   while number != 0 {
     println!("{}!", number);

     number = number - 1;
   }

   println!("LIFTOFF!!!");

   let arr = [10, 20, 30, 40, 50];

   for elem in arr.iter() {
     println!("the value is :{}", elem);
   }

   for number in (1..4).rev() {
     println!("{}!", number);
   }
}