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

fn main() {
    // 泛型, 被设计用来处理重复的逻辑的工具

    let number_list = vec![1, 2, 10, 3];
    println!("The largest number is {}", lagrest(&number_list));

    let number_list = vec![203, 89, 10, 8392];
    println!("The largest number is {}", lagrest(&number_list));
}