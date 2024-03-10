fn hello_world() {
  println!("Hello, world!");
}

// 基本的な型
fn basic_type() {
  let x = 12; // デフォルトでは i32
  let a = 12u8;
  let b = 4.3; // デフォルトでは f64
  let c = 4.3f32; // 浮動小数点数型
  let bv = true;
  let t = (13, false);
  let sentence = "hello world!";

  // 数値や関数や参照など，型の実体はすべてオブジェクトなので、
  // 式が返す値もまたオブジェクトになります．例えば， 1 という値も数値オブジェクトであり，1 == {1} という関係になる
  println!("=={}==", " 基本的な型のサンプル ");
  println!(
    "i32: {} u8: {} f64: {} f32: {} bool: {} tupple_value: {} tupple_value: {} sentence: {}",
    x, a, b, c, bv, t.0, t.1, sentence
  );
}

fn array_sumple() {
  // 配列のデータ型は固定長
  let nums: [i32; 5] = [1, 2, 3, 4, 5];
  println!("=={}==", " 配列のサンプル ");
  println!("{:?}", nums);
  println!("{}", nums[1]);
}

// 戻り値は -> で指定
fn add(x: i32, y: i32) -> i32 {
  println!("=={}==", " 関数のサンプル ");
  return x + y;
}

// 関数の戻り値はタプルで指定することもできる
fn swap(x: i32, y: i32) -> (i32, i32) {
  return (y, x);
}

// 戻り値がない場合は 空のタプルを返すこともできる
fn make_nothing() -> () {
  return ();
}

// 定数
const PI: f32 = 3.14159;

fn main() {
  hello_world();
  basic_type();
  array_sumple();
  println!("{}", add(33, 33));

  let swap_result = swap(1, 2);
  println!("{} {}", swap_result.0, swap_result.1);

  // タプルを2つの変数に分解することもできる
  let (a, b) = swap(3, 4);
  println!("{} {}", a, b);

  let empty_tuple = make_nothing();
  println!("empty tuple: {:?}", empty_tuple);

  // mutをつけることで変数を変更可能にする
  let mut x = 0.1;
  print!("{}", x);
  x = 0.2;
  print!("{}", x);
  print!("{}", PI);
}
