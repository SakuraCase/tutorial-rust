#[derive(Debug)]
struct User {
    username: String,
    email: String,
    active: bool,
}

#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
    };
    println!("user1: {:?}", user1);

    let mut user2 = User { ..user1 };
    user2.email = String::from("hogehoge@example.com");
    user2.username.push_str("hoge");
    user2.active = false;
    // println!("{:?}", user1); // error borrow of partially moved value: `user1`
    println!("user2: {:?}", user2);

    let user3 = User {
        email: user2.email.clone(),
        username: user2.username.clone(),
        ..user2
    };
    user2.username = String::from("someusername124");
    println!("user2: {:?}", user2);
    println!("user3: {:?}", user3);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("{:?}, {:?}, {}", black, origin, origin.1);
}
