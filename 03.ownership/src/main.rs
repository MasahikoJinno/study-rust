fn main() {
    // 所有権の移動(move)
    let x = String::from("hello world.");
    let y = x; // ここでxの中身(実際はポインタ)がyに移り、xは空になる。
    println!("{}", y);
    // 3行目で所有権を移動(move)しているので、借用(borrow)出来ず
    // ↓はコンパイルエラー
    // println!("{}", x);

    // 所有権の借用(borrow)と参照(reference)
    let s = String::from("hello");
    // &は参照(reference)の意味、
    // &をつけると string_length 関数に対して所有権を渡さないことができる
    let len = string_length(&s);
    println!("s={}, len={}", s, len);
}

fn string_length(s: &String) -> usize {
    s.len()
}