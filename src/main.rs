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

    let dice_roll = 9;
    match dice_roll {
        3 => 3,
        7 => 7,
        o => o,
    };
    // 注意使用other来匹配所有特殊列出来的值. 这样就满足了match必须穷尽的值.
    // 注意匹配的时候分支的顺序,因为match是通过顺寻来匹配的.

    // rust 提供_模式,当我们不想要使用通配模式获取的值时,可以使用_,可以匹配任意值,但是不绑定到该值上.(不绑定表示忽略匹配的值),而后在_对应的代码块中执行操作.
}