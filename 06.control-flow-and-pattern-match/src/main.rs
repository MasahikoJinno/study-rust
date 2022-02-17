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
}

fn equal(a: i32, b: i32) -> bool {
    a == b
}
