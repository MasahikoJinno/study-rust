# 所有権（ownership）

## 概要

- 所有権はRustの最もユニークな機能
- これのおかげでGC無しで安全性を担保できる

## 移動（move）

変数から変数に所有権を移すことを移動（move）と呼ぶ

```rust
let x = String::from("hello world.");

// ここでxの中身(実際はポインタ)がyに移り、xは空になる。
let y = x;

// 所有権がxからyに写っているので、xを参照しようとするとコンパイルエラー
println!("{}", x); // コンパイルエラーになる
```

## 借用（borrow）

関数に対して変数の所有権を移動（move）してしまうと、後続の処理で変数を利用できなくなる

```rust
fn main() {
    let x = String::from("Hello")
    // 変数xをlengthに移動
    let len = length(x);
    
    // lengthに所有権を移動しているので、コンパイルエラー
    println!("x={}", x);
    
}

fn length(s: String) -> usize {
    s.len()
}
```

所有権を移動（move）させずに、参照（reference）を渡すことを「借用（borrow）」と呼称する。

借用（borrow）をすることで、関数に引数を渡した後も変数に所有権が残る。

```rust
fn main() {
    let x = String::from("Hello")
    // &をつけることで参照（reference）となり、length関数に所有権を移動させないことができる
    let len = length(&x);
    
    // 所有権を移動させてないので、コンパイルエラーとならない
    println!("x={}", x);
    
}

fn length(s: &String) -> usize {
    s.len()
}
```
