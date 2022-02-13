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

関数が引数を通じて変数の中身を移動（move）させることを借用（borrow）と呼ぶ

```rust
fn main() {
    let x = String::from("Hello")
    // 変数xから所有権を借用
    let len = length(x);
    
    // lengthに所有権を移動しているので、コンパイルエラー
    println!("x={}", x);
    
}

fn length(s: String) -> usize {
    s.len()
}
```

## 参照（reference）

上記のように関数に変数を渡すだけで移動をしてしまうと面倒なので、参照（reference）を使って値を参照することができる。

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
