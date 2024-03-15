fn conditions() {
  let x = 42;
  if x < 42 {
    println!("42 より小さい");
  } else if x == 42 {
    println!("42 に等しい");
  } else {
    println!("42 より大きい");
  }
}

fn loop_fn() {
  let mut counter = 0;
  let result = loop {
    counter += 1;
    if counter == 10 {
      break counter * 2;
    }
  };

  println!("result: {}", result);
}

fn while_loop() {
  let mut number = 3;
  while number != 0 {
    println!("{}!", number);
    number -= 1;
  }
  println!("LIFTOFF!!!");
}

fn for_loop() {
  let a = [10, 20, 30, 40, 50];

  for element in a.iter() {
    println!("the value is: {}", element);
  }
}

pub fn main() {
  println!("=={}==", " Practice2 ");

  let a = "test";
  println!("{}", a);

  // {} を使ってブロックを作ることで，スコープを作ることができる
  {
    let a = "test2";
    println!("{}", a);
  }

  conditions();
  loop_fn();
  while_loop();
  for_loop();
}
