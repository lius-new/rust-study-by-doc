fn main() {
    // Option 类型, 其表示要么有值(Some),要么无值(None).
    // Option是一个枚举类型,包含了Some 和 None
    // Option是十分有用的枚举,其被包含在prelude中,使用它甚至不用显示引入

    let some_number = Some(5);
    let some_chat = Some('c');
    let some_nonoe: Option<i32> = None;
    // 当如果类型是Some那么就意味这是一个非空值,当如果是None那么就是一个空值
}