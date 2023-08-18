fn main() {
    // 泛型, 被设计用来处理重复的逻辑的工具

    let number_list = vec![1, 2, 10, 3];
    let mut largest = &number_list[0];

    for number in number_list.iter() {
        if number > largest {
            largest = number;
        }
    }
    println!("The largest number is {}", largest);

    let number_list = vec![203, 89, 10, 8392];
    let mut largest = &number_list[0];

    for number in number_list.iter() {
        if number > largest {
            largest = number;
        }
    }
    println!("The largest number is {}", largest);
}