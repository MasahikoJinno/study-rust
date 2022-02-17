fn main() {
    let a = 10;
    let b = 20;

    if a < b {
        println!("a < b");
    } else if a == b {
        println!("a == b");
    } else {
        println!("a > b");
    }

    if a == 10 && b == 20 {
        println!("and");
    }

    if a == 10 || b == 20 {
        println!("or");
    }

    // ifの条件式に関数を指定できる
    if equal(22, 22) {
        println!("equal returned true");
    }

    let x = if a > b {
        "a > b"
    } else {
        "else"
    };

    println!("x={}", x);

    // ベクターの数分繰り返し処理を行う
    let v = vec![10, 20, 30, 40, 50];
    print!("v=[");
    for i in &v {
        let x: i32 = *i;
        print!("{}, ", x);
    }
    println!("]");

    // イテレータを使用して繰り返し処理をする
    let v = vec![10, 20, 30, 40, 50];
    print!("v=[");
    for i in v.iter() {
        let x: i32 = *i;
        print!("{}, ", x);
    }
    println!("]");

    // イテレータを使用して、インデックス付きの繰り返し処理をする
    let v = vec![10, 20, 30, 40, 50];
    print!("v=[");
    for (i, x) in v.iter().enumerate() {
        print!("{}:{}, ", i, x);
    }
    println!("]");

    // 指定回数繰り返す
    for i in 0..10 {
        print!("{} ", i);
    }
    println!("");

    // 負の数でも良いっぽい
    for i in -5..-1 {
        print!("{} ", i);
    }
    println!("");

    // while
    print!("while = ");
    let mut i = 0;
    while i < 10 {
        print!("{} ", i);
        i += 1;
    }
    println!("");

    // loop
    print!("loop = ");
    let mut i = 0;
    loop {
        if i >= 10 {
            break;
        }
        print!("{} ", i);
        i += 1;
    }
    println!("");
}

fn equal(a: i32, b: i32) -> bool {
    a == b
}
