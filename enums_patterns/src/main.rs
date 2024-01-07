enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// #[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            // println!("State quarter from {:?}!", state);
            25
        }
    }
}

impl Message {
    fn call(&self) -> u32 {
        // method body would be defined here
        match &self {
            Message::Quit => 1,
            Message::Move { x, y } => todo!(),
            Message::Write(_) => todo!(),
            Message::ChangeColor(_, _, _) => todo!(),
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);


    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // Same as
    // let config_max = Some(3u8);
    // if let Some(max) = config_max {
    //     println!("The maximum is configured to be {}", max);
    // }
}
