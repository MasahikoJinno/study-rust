fn main() {
    // セミコロンが文の区切りになる
    let a = 10; let b = 20; println!("a={}, b={}", a, b);

    // 文は複数行にまたがることができる
    let a
        = 10;
    let b
        = 20;
    println!(
        "a={}, b={}",
        a,
        b
    );

    // 式は値を返す ※ 10 + 20の部分が式
    let a = 10 + 20;
    // ブロックも値を返すので式にできる。（Kotlin感がある）
    // { 10 + 20; };のようにセミコロンをつけると「文」になるので、bを借用しようとするとコンパイルエラーとなる
    let b = { 10 + 20 };

    println!("a={}, b={}", a, b);

    // 関数は式になる
    let x = add(20, 30);

    println!("x={}", x);

    // if文はbool値をもつ式を条件につけられる
    let a = 10;
    if a > 0 {
        println!("a={}", a);
    }

    // 関数に文字列を渡す
    str_param("rust");

    // 関数に文字列を渡して、加工された文字列を受け取る
    let r = str_param_and_return("rust");
    println!("r={}", r);

    // 整数のベクターを渡して合計の数値を受け取る
    let r = sum(&vec![1, 2, 3, 4, 5]);
    println!("r={}", r);

    // 自然数を渡してその数のベクターを返す
    let v = return_vec(10);
    for i in v {
        print!("{} ", i);
    }
    println!("");
}

fn add(x: i32, y: i32) -> i32 {
    // セミコロンがついてないのは式として値を返すため
    // x + y; としてしまうと戻り値の無い（厳密には `()` を返す）関数になる
    x + y
}

// 文字列を受け取る関数
fn str_param(s: &str) {
    println!("s={}", s);
}

// 文字列を受け取り、文字列を加工して返す関数
fn str_param_and_return(s: &str) -> String {
    println!("s={}", s);
    let r = format!("hello {} world!", s);
    r
}

// ベクターを受け取り、合計数値を返す関数
fn sum(v: &Vec<i32>) -> i32 {
    println!("called sum");
    let mut sum = 0;
    for i in v {
        sum += i;
    }
    sum
}

fn return_vec(max: i32) -> Vec<i32> {
    println!("called return_vec");
    let mut v = Vec::new();
    for i in 0..max {
        v.push(i);
    }
    v
}