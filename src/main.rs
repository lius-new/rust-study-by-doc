fn main() {
    let number = 3;

    // 注意, if 判断的值的结果必须是bool类型.
    if number > 4 {
        println!("win")
    } else {
        println!("win")
    };

    // 实际上,if表达式,那么他也有返回值的
    // 此时其对应的值并不明确,因为可能条件结果为false
    // let number = if number > 4 {
    //     4
    // };

    // 此if返回值的类型是明确的. 且他们的类型应该是一样的,这样rust才知道其返回值的类型.
    let number = if number > 4 {
        4
    } else {
        5
    };
    assert_eq!(number, 5);
}
