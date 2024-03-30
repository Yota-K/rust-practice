struct User {
  name: String,
  age: u32,
}

struct Apple {
  size: u32,
}

impl Apple {
  fn get_size(&self) -> u32 {
    self.size
  }
}

pub fn main() {
  println!("=={}==", " Practice3 ");

  let name = String::from("John");
  let age = 30;
  let user1 = User { name, age };
  println!("User: {}", user1.name);
  println!("User: {}", user1.age);

  let apple = Apple { size: 10 };
  let apple_size = apple.get_size();

  println!("Apple size: {}", apple_size);
}
