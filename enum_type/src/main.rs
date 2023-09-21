fn main() {
    let coin1 = Coin::Penny;
    let coin2 = Coin::Quarter(UsState::Alaska);
    value_in_cents(coin1);
    value_in_cents(coin2);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?} {:?} {:?}", five, six, none);

    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => print!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}

#[derive(Debug)]
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

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        },
        Coin::Nickel => 5, // 5 cents
        Coin::Dime => 10, // 10 cents
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}