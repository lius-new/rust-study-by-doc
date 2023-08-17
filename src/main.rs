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
        Coin::Penny => {
            println!("hello world");
            1
        }
        Coin::Nickel => 2,
        Coin::Dime => 3,
        Coin::Quarter(usState) => {
            println!("State quarter from {:?}!", state);
            4
        }
    }
}

fn main() {}