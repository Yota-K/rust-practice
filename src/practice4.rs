// fn failed_pattern() {
//   // 整数値やブール値などの値はコピーになるので所有権は移動しない
//   let int1 = 100;
//   let int2 = int1;
//   let bool1 = true;
//   let bool2 = bool1;
//
//   let s1 = String::from("hello");
//   // Rustでは代入(=)や関数の引数や戻り値などで値を受け渡すと所有権が移動する
//   // 変数s1から変数s2に所有権の移動が発生（一度に存在できる所有者は1人だけ）
//   // 所有権が移動することをムーブという
//   let s2 = s1;
//   println!("s1: {}", s1);
// }
//
// fn failed_pattern2() {
//   let mut s = String::from("hello");
//
//   // 同じスコープ内でsのミュータブルな参照を2つ宣言しているためコンパイルエラーが起きる
//   let r1 = &mut s;
//   let r2 = &mut s;
//   println!("{}, {}", r1, r2);
// }

fn scope_pattern() {
  let s1 = String::from("hello");

  {
    let s2 = String::from("hi");
    println!("{}, world!", s2);
  } // 値"hi"の所有者であるs2がスコープが外れるので、"hi"は破棄される

  // 値"hello"の所有者であるs1がスコープが外れるので、"hello"は破棄される
  println!("{}, world!", s1);
}

fn calculate_length(s: &str) -> usize {
  s.len()
}

fn change_mut(some_string: &mut String) {
  some_string.push_str(", world");
}
fn change(some_string: String) {
  println!("{} world", some_string);
}

pub fn main() {
  println!("=={}==", " Practice4 ");
  scope_pattern();

  let mut s1 = String::from("hello");
  let s2 = String::from("world");
  // &mut 参照を渡しているので書き込み可能。所有権は移動しない
  change_mut(&mut s1);
  change(s2);

  // & で参照させているので読み取りのみ可能。所有権は移動しない
  let len = calculate_length(&s1);
  println!("The length of '{}' is {}.", s1, len);

  // failed_pattern();
  // failed_pattern2();
}
