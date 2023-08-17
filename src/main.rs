#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

fn main() {
    let width = 30;
    let height = 50;

    println!("The area of the rectangle is {} square pixels.", area(width, height));
    println!("The area of the rectangle is {} square pixels.", area_v2((height, width)));
    println!("The area of the rectangle is {} square pixels.", area_v3((height, width)));

    let rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    // println!("The area of the rectangle is {} square pixels.", area_v4(rectangle));
    // println!("{:?}", rectangle); // value borrowed here after move

    let rectangle2 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area of the rectangle is {} square pixels.", area_v5(&rectangle2));
    println!("{:?}", rectangle); // value borrowed here after move

    // `Rectangle` cannot be formatted with the default formatter
    // the trait `std::fmt::Display` is not implemented for `Rectangle`
    // 无法直接打印出来是因为结构体没有实现默认的Display, 默认只是为一些已经固定内存大小的值来创建(基本的存在栈中的数据)
    println!("{}", rectangle2);

    // 使用:? 或者:#? 指示符告诉println!宏我们采用Debug的输出方式,Debug是一种trait,允许我们直接打印结构体
    println!("{:?}", rectangle2);


}

fn area(width: i32, height: i32) -> i32 {
    width * height
}

fn area_v2((width, height): (i32, i32)) -> i32 {
    width * height
}

fn area_v3(dimensions: (i32, i32)) -> i32 {
    dimensions.0 * dimensions.1
}

// 发生了移动,因为传入的是所有者而非所有者的引用(借用)
fn area_v4(rectangle: Rectangle) -> i32 {
    rectangle.width * rectangle.height
}

fn area_v5(rectangle: &Rectangle) -> i32 {
    rectangle.width * rectangle.height
}
