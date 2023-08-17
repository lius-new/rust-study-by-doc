fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(x) => Some(x + 1),
        _ => None
    }
}

fn main() {
    let five: Option<i32> = Some(5);

    let five_plus = plus_one(five);

    let res = plus_one(None);
}