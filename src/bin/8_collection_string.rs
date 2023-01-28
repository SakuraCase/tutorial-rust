fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    // let s3 = s1 + s2;   // error  mismatched types
    // let s3 = &s1 + &s2; // error cannot add `&String` to `&String`
    // s1.add(&s2);        // error no method named `add` found for struct `String` in the current scope
    // println!("{}", s1); // error borrow of moved value: `s1`
    println!("{}", s3);
    let s4 = format!("Hello! {}", s3);
    println!("{}", s4);

    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("{}", s);
    // let s = &hello[0..1]; // thread 'main' panicked at 'byte index 1 is not a char boundary;
    // println!("{}", s); //error

    for b in hello.chars() {
        print!("{}", b);
    }
}
