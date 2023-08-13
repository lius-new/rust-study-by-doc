fn main() {
    let mut i = 0;
    let r = loop {
        println!("2");
        i + 1;
        if i == 10 { break i; }
    };

    // 多个loop循环
    // 可以通过`'loop_token: `来指定循环的标识循环,以供在嵌套循环中区分循环
    let mut counter = 0;
    let t = 'other: loop {
        counter += 1;
        println!("other");
        'inner: loop {
            println!("inner");
            if counter == 5 {
                break 'other 2;
            }
        }
    };
}
