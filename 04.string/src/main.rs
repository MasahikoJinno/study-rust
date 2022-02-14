fn main() {
    let s = "hello rust world.";

    // 部分文字列を取得
    // ※添字は文字のindexではなくByteの位置なのでマルチバイト文字を使うときは注意
    let hello = &s[0..5];
    let world = &s[11..];
    println!("hello={}", hello);
    println!("world={}", world);

    // 長さを取得する
    let len = s.len();
    println!("len={}", len);

    // 文字列連結
    let mut s = String::new();
    s.push_str("hello ");
    s.push_str("rust ");
    s.push_str("world.");
    println!("s={}", s);

    // format!マクロで連結
    let hello = "HELLO";
    let rust = "RUST";
    let world = "WORLD";
    let s = format!("{} {} {}.", hello, rust, world);
    println!("s={}", s);

    // 日本語もつかえる
    let s = "こんにちはRustの世界。";
    // .len()は文字数ではなくByte数を返す
    // 13文字だが、↓は31となる。
    let len = s.len();
    println!("s={}, len={}", s, len);

    // ベクターを使った正しいマルチバイト文字の操作（絶対現場では使わない）
    let s = "こんにちはRustの世界。";
    let mut v: Vec<char> = Vec::new();
    for c in s.chars() {
        v.push(c);
    }
    let v = &v[0..5];
    let mut s = String::new();
    for c in v {
        s.push(*c);
    }
    println!("s={}", s);
}
