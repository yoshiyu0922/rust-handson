fn main() {
    println!("Hello, world!");

    // 変数と可変性
    let x = 5;
    println!("The value of x is: {}", x);
    // x = 6; // immutableのためコンパイルエラーになる mut を付け加えれば良い
    // println!("The value of x is: {}", x);

    let mut y = 5;
    println!("The value of y is: {}", y);
    y = add1(y); 
    println!("The value of add1(y) is: {}", y);
    y = add2(y); 
    println!("The value of add2(y) is: {}", y);
}

fn add1(x: i32) -> i32 { // 戻り値は必ず型を指定する
  x + 1 // 戻り値の返し方は２パターンある
}

fn add2(x: i32) -> i32 {
  return x + 2;
}
