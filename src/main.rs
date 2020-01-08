fn main() {
    println!("Hello, world!");

    // 変数と可変性
    let x = 5;
    let _z = x; // 使用されない変数はprefixに"_"をつける。
    println!("The value of x is: {}", x);
    // x = 6; // immutableのためコンパイルエラーになる mut を付け加えれば良い
    // println!("The value of x is: {}", x);

    // 関数
    let mut y = 5;
    println!("The value of y is: {}", y);
    y = add1(y); 
    println!("The value of add1(y) is: {}", y);
    y = add2(y); 
    println!("The value of add2(y) is: {}", y);

    // 変数のスコープ
    let mut scope;
    { 
      let scope_x = 5;
      scope = &scope_x; // 所有権を渡すのではなく、参照できるようにする
      let scope_string = String::from("hello");
      println!("The value of scope_x is: {}", scope_x);
      println!("The value of scope_string is: {}", scope_string);
    }
    // println!("The value of scope_x is: {}", scope_x); // スコープ外
    // println!("The value of scope is: {}", scope); // 参照なのでエラーが起こる。所有権を渡せば実行できる

    // 変数を別の変数に代入する場合、スタックの場合はコピーされるが、ヒープの場合は所有権が奪われて参照できなくなる
    println!("The value of x is: {}", x);
    let stack = 1;
    let heap = String::from("test");
    println!("The value of heap is: {}", heap);
    let len1 = calulate_length(heap); 
    println!("The value of len1 is: {}", len1);
    // let len2 = calulate_length(heap);// ヒープの場合は所有権が奪われて、heapが参照できなくなるのでコンパイルエラーになる

    // 所有権が奪われないために参照を関数に渡すようにする => 借用
    let heap2 = String::from("test");
    let len2 = calulate_length2(&heap2);
    println!("The value of len2 is: {}", len2);

    let num1 = add1(stack);
    let num2 = add1(stack); // スタックの場合はコンパイルエラーにならない
    println!("The value of num1 is: {}", num1);
    println!("The value of num2 is: {}", num2);

    let s1 = String::from("hello");
    // let s2 = s1;//"hello"の所有権がs1からs2に移動
    println!("The value of s1 is: {}",s1);
}

fn add1(x: i32) -> i32 { // 戻り値は必ず型を指定する
  x + 1 // 戻り値の返し方は２パターンある
}

fn add2(x: i32) -> i32 {
  return x + 2;
}

fn calulate_length(s: String) -> usize {
  let length = s.len();
  length
}

// 所有権が奪われないために参照を引数とする
fn calulate_length2(s: &String) -> usize {
  let length = s.len();
  length
}
