struct User {
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool
}

fn main() {

  let user1 = build_user(String::from("user1@qq.com"), String::from("user1"));

  let user2 = User {
    email: String::from("user2@qq.com"),
    username: String::from("user2"),
    ..user1
  };
}

fn build_user(email: String, username: String) -> User {
  User {
    email,
    username,
    active: true,
    sign_in_count: 1
  }
}