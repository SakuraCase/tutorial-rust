fn main() {
    another_function(5);
    println!("{}", five());
    test_types();
    test_if();
    test_loop();
    test();
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn five() -> i32 {
    5
}

fn test() {
    let hoge = {
        let a = 5;
        let b = 2;
        a * b
    };
    println!("{}", hoge);
}

fn test_if() {
    println!("--- test_if()");
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);

    // error mismatched types
    // if number {
    //     println!("number was three");
    // }
}

fn test_loop() {
    println!("--- test_loop()");
    let mut number = 1;

    loop {
        number += 1;
        if number == 3 {
            break;
        }
    }

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {}", element);
    }
}

fn test_types() {
    println!("--- test_types()");
    let mut x = 5;
    println!("{}", x); // 5
    x = 6;
    println!("{}", x); // 6

    // println!(x); // error
    println!("hoge");

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x); // 12
    }
    println!("{}", x); // 6

    let x = "xx"; // &str
    println!("{}", x);
    // println!(x); // error
    let x = 'x'; //char
    println!("{}", x);
    // println!(x); // error

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("{:?}", tup); // (500, 6.4, 1)
    let tup = (500, 6.4, 1);
    println!("{}, {}, {}", x, y, z); // 500, 6.4, 1

    println!("{}", tup.0); // 500
    println!("{}", tup.1); // 6.4

    let x = [1, 2, 3, 4, 5];
    println!("{:?}", x); // [1, 2, 3, 4, 5]
    println!("{}", x[1]); // 2

    // println!("The value of x is: {}", x[7]); // error
    // println!("The value of x is: {}", x.1); // error

    // let (x1, x2, x3, x4, x5) = x; // error mismatched types
    let [x1, x2, x3, x4, x5] = x;
    println!("{}, {}, {}, {}, {}", x1, x2, x3, x4, x5); // 1, 2, 3, 4, 5

    let x = [1; 5];
    println!("{:?}", x);
}
