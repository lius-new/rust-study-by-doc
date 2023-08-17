fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(x) => Some(x + 1),
        _ => None
    }
}

// 该段代码的差别在于match不匹配_(None), 此时的match没有匹配到所有的可能(match 是exhaustive(穷尽)),因此无法通过编译.
fn plus_two(x: Option<i32>) -> Option<i32> {
    match x {
        Some(x) => Some(x + 1),
    }
}

fn main() {
    let five: Option<i32> = Some(5);

    let five_plus = plus_one(five);

    let res = plus_one(None);
}