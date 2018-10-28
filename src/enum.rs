#[derive(Debug)]
enum IpAddrKind {
  V4,
  V6
}

struct IpAddr {
  kind: IpAddrKind,
  address: String
}

fn main() {
  let four = IpAddrKind::V4;
  let six = IpAddrKind::V6;

  println!("{:?}", four);

  route(IpAddrKind::V4);
  route(IpAddrKind::V6);

  let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1")
  };
}

fn route(op_type: IpAddrKind) { }