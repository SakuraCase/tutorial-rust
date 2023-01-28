fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    println!("{:?}", v);
    let mut v = vec![1, 2, 3];
    let third = &v[2];
    // v.push(4); // error  cannot borrow `v` as mutable because it is also borrowed as immutable
    println!("{}", third);
    v.push(4);

    println!("{:?}", v);
    match v.get(2) {
        Some(n) => println!("{}", n),
        None => println!("none"),
    }
    match v.get(10) {
        Some(n) => println!("{}", n),
        None => println!("none"),
    }

    for i in &v {
        println!("{}", i);
    }
    for i in &mut v {
        *i += 50;
    }
    println!("{:?}", v);

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("{:?}", row);
}
