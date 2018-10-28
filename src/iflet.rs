enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
  Alabama,
  Alaska
}

fn main() {
  let some_u8_value = Some(0u8);
  if let Some(3) = some_u8_value {
    println!("three");
  }

  let mut count = 0;
  let coin = Coin::Quarter(UsState::Alaska);
  if let Coin::Quarter(state) = coin {
    println!("State qurter from {:?}!", state);
  } else {
    count += 1;
  }
}