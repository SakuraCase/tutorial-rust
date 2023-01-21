fn main() {
    // --------------------------- 4.1
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    // let s2 = s; // error borrow of moved value: `s`
    let mut s2 = s.clone();
    println!("{} {}", s, s2);
    s2.push_str("!!");
    println!("{} {}", s, s2);

    s2 = takes_and_gives_back(s2);
    println!("{}", s2);

    // s2の所有権は関数にmove
    takes_ownership(s2);
    // println!("{}", s2); // error borrow of moved value: `s2`

    // スカラー値などであればコピー可能
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    let x = (2, 4);
    let y = x;
    println!("x = {:?}, y = {:?}", x, y);

    // let x = (2, String::from("x"));
    // let y = x; // error borrow of moved value: `x`
    // println!("x = {:?}, y = {:?}", x, y);

    // --------------------------- 4.2
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");
    change(&mut s);

    // 可変参照はスコープ内1回
    let r1 = &mut s;
    // let r2 = &mut s; // error cannot borrow `s` as mutable more than once at a time
    println!("{}", r1);

    // 可変参照と不変参照同時も不可
    let mut s = String::from("hello");
    // let r1 = &s; // cannot borrow `s` as mutable because it is also borrowed as immutable
    let r2 = &mut s;
    println!("{}", r2);

    let reference_to_nothing = dangle();
    println!("{}", reference_to_nothing);

    // --------------------------- 4.3
    let s = String::from("hello world");
    let word = first_word(&s);
    // s.clear(); // error cannot borrow `s` as mutable because it is also borrowed as immutable
    println!("the first word is: {}", word);
    println!("{}, {}, {}, {}", &s[..], &s[0..5], &s[0..], &s[..5]);
    let word = first_word(word);
    println!("the first word is: {}", word);
}

fn takes_ownership(some_string: String) {
    println!("takes_ownership: {}", some_string);
}

fn takes_and_gives_back(a_string: String) -> String {
    println!("a_ownership: {}", a_string);
    a_string.replace("world", "rust")
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// fn change_error(some_string: &String) {
//     some_string.push_str(", world"); // error cannot borrow `*some_string` as mutable, as it is behind a `&` reference
// }
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle_error() -> &String { // missing lifetime specifier
//     let s = String::from("hello");
//     &s
// }
fn dangle() -> String {
    let s = String::from("hello");
    println!("{}", s);
    s
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
