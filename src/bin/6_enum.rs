struct Ipv4Addr(u8, u8, u8, u8);

struct Ipv6Addr;

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
        println!("{:?}", &self)
    }
}

fn main() {
    // ----- 6.1
    let four = IpAddr::V4(Ipv4Addr(127, 0, 0, 1));
    let six = IpAddr::V6(Ipv6Addr);
    // println!("{}", four.1); // error no field `1` on type `IpAddr`
    for ip in [&four, &six] {
        match ip {
            IpAddr::V4(ipv4) => println!("{}.{}.{}.{}", ipv4.0, ipv4.1, ipv4.2, ipv4.3),
            IpAddr::V6(_) => println!("ipv6"),
        }
    }
    if let IpAddr::V4(ipv4) = four {
        println!("if let {}.{}.{}.{}", ipv4.0, ipv4.1, ipv4.2, ipv4.3)
    }
    if let IpAddr::V4(_) = six {
        println!("if let six")
    }
    if let IpAddr::V4(_) = six {
        println!("if let v4")
    } else {
        println!("if let v4 else")
    }

    let m = Message::Quit;
    m.call();
    let m = Message::Write(String::from("hello"));
    m.call();
    let m = Message::Move { x: 1, y: 2 };
    m.call();
    let m = Message::ChangeColor(1, 2, 3);
    m.call();

    // ----- 6.2
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            Some(i) => Some(i + 1),
            _ => None,
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?}", six);
    println!("{:?}", none);
}
