// 限制T的类型只对实现了 PartialOrd类型有效
fn lagrest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for number in list.iter() {
        if number > largest {
            largest = number;
        }
    }
    largest
}

// 结构体的泛型,注意此时结构体中字段的类型为泛型T,这样也就意味x,y字段的类型都是一样的。 可以通过设置两个泛型类型
struct Point<T, Y> {
    x: T,
    y: Y,
}


enum Options<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn main() {
    // 泛型, 被设计用来处理重复的逻辑的工具

    let number_list = vec![1, 2, 10, 3];
    println!("The largest number is {}", lagrest(&number_list));

    let number_list = vec![203, 89, 10, 8392];
    println!("The largest number is {}", lagrest(&number_list));

    let integer = Point { x: 1, y: 2 };
    let float = Point { x: 1.0, y: 2.0 };
}