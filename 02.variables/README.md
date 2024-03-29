# 変数と型

## 概要
- 変数は `let` で宣言する
- 型推論ができる
- 型指定は変数名に `:[type]` の形式で指定する
- 再代入不可
  - 再代入させたいときは `mut` を付ける必要がある
  - `mut` を着けずに `let` のみで宣言することを束縛（bind）と呼ぶ
- 変数は同じ名前で何度も宣言できる
  - シャドーイングと呼ぶ
  
## スカラー型（Scalar Types）
### 整数型（Integer Types）
#### 型
| サイズ     | 符号付き  | 符号なし  | 備考                                |
|---------|-------|-------|-----------------------------------|
| 8-bit   | i8    | u8    |                                   |
| 16-bit  | i16   | u16   |                                   |
| 32-bit  | i32   | u32   | i32が基準型となる（型推論で整数型と推論される場合i32になる） |
| 64-bit  | i64   | u64   |                                   |
| 128-bit | i128  | u128  |                                   |
| arch    | isize | usize | 実行環境のアーキテクチャに依存                   |

#### リテラル
| 数値リテラル        | 例           |
|---------------|-------------|
| Decimal(10進数) | 12_345      |
| Hex(16進数)     | 0xff        |
| Octal(8進数)    | 0o77        |
| Binary(2進数)   | 0b1111_0000 |
| Byte(1Byte)   | b'A'        |

### 浮動小数点型（Floating-Point Types）

| サイズ    | 型   | 備考                                   |
|--------|-----|--------------------------------------|
| 32-bit | f32 |                                      |
| 64-bit | f64 | f64が基準型となる（型推論で浮動小数点型と推論される場合f64になる） |

### 論理値型（The Boolean Type）

- `bool` が論理値型となる。※ `boolean` でも可

### 文字型（The Character Type）

- `char` が文字型となる
- Rustのchar型は、ユニコードのスカラー値を表します
  - マルチバイト文字や絵文字も使える（C言語などでは使えない）
- リテラルはシングルクォート( `''` ) で囲う

## 複合型(Compound Types)
### タプル型（The Tuple Type）

- `tuple` は複数の型の何らかの値を一つの複合型にまとめ上げる一般的な手段
- `()` で値を列挙する
- `.[index]` で値を参照できる

```rust
let t = ('masahiko', 32)
// ↓の構文を分配と呼ぶ。JSの分割代入みたいなもん。
let (name, age) = t 
println!("name={}, age={}", t.0, t.1)
```

### 配列型（The Array Type）

- 他の言語の配列とほぼ同じ
- C/C++/Javaのように固定長となり、サイズを後から可変させることは出来ない

