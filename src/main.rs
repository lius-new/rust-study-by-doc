enum Temp {
    Type1(i32),
    Type2(u32),
    Type3(&'static str),
}

enum Temp2 {
    Type3(&'static str),
}

fn main() {
    let config_max = Some(3u8);
    // 当如果只关心Some时的代码,那么就要写很多 _=>...这样的代码
    let res = match config_max {
        Some(max) => {
            println!("The maximum is configured to be {}", max);
            Some(max)
        }
        _ => None,
    };

    // if let 语法等号分割一个模式和表达式.他的工作方式和match是相同的. 这里的表达式对应的是match中的第一个分支
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    // if let也可以配合else使用,这也就相当于_=>/other=>了
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    } else {
        println!("The maximum is not exist");
    }
    let temp = Temp::Type3("hello");

    if let Temp::Type3(value1) = temp {
        if let value = value1 { // 嵌套的if let 模式匹配
            println!("{:?}", value);
        } else {}
    } else {
        println!()
    }
}