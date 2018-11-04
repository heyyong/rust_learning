pub trait Summarizable {
  fn summary(&self) -> String;
}

pub struct NewsArticle {
  pub headline: String,
  pub location: String,
  pub author: String,
  pub content: String,
}

impl Summarizable for NewsArticle {
  fn summary(&self) -> String {
    format!("{}, by {} ({})", self.headline, self.content, self.author)
  }
}

pub struct Tweet {
  pub username: String,
  pub content: String,
  pub reply: bool,
  pub retweet: bool
}

impl Summarizable for Tweet {
  fn summary(&self) -> String {
    format!("{}: {}", self.username, self.content)
  }
}

fn main() {
  let tweet = Tweet {
    username: "asdf".to_string(),
    content: "just a test".to_string(),
    reply: false,
    retweet: false
  };

  println!("{}", tweet.summary());
}