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
}

fn add(x: i32, y: i32) -> i32 {
    // セミコロンがついてないのは式として値を返すため
    // x + y; としてしまうと戻り値の無い（厳密には `()` を返す）関数になる
    x + y
}