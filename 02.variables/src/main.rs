fn main() {
    // 変数宣言
    // 型推論を利用
    let name = "masahiko jinno";
    let age = 32;
    let x = 100.2;
    println!("name={}, age={}, x={}", name, age, x);

    // 型指定
    let name: &str = "masahiko jinno";
    let age: i32 = 32;
    let x: f64 = 100.2;
    println!("{}, {}, {}", name, age, x);

    // 文字型
    let c1 = 'A';
    let c2 = 'あ'; // マルチバイトも文字型で扱える
    let c3 = '🦀'; // 絵文字も使える
    let c4: char = '🍤'; // 型指定
    println!("c1={}, c2={}, c3={}, c4={}", c1, c2, c3, c4);

    // 文字列
    let s1 = String::from("Hello");
    let s2 = String::from("Rust");
    let s3 = String::from("world.");
    let ss1 = s1 + " " + &s2 + " " + &s3;
    // こういう書き方もできる、ただs1はすでにmoveしてるので↓を書くとコンパイルエラー
    // let ss2 = format!("{} {} {}", s1, s2, s3);
    println!("ss1={}", ss1);

    // tuple
    let t = (name, age);
    println!("name={}, age={}", t.0, t.1);

    // array
    let a = ["🦀", "🍤"];
    println!("{}, {}", a[0], a[1]);
}