fn main() {
    // 使用while遍历
    let arr = [10; 5];
    let mut index = 0;
    while index < 5 {
        println!("{}", arr[index]);
        index += 1
    }

    // 使用for
    for item in arr {
        println!("{}", item);
    }

    // 使用1..=3来生成一组数据,并进行遍历
    for item in 1..=3 {
        println!("{item}")
    }
    // 使用1..=3来生成一组数据,然后反转,并进行遍历
    for item in (1..=3).rev() {
        println!("{item}")
    }

    // TODO: RangeInclusive<i32> 类型 ,这是一种什么类型?
    let item = (1..=3);
}
