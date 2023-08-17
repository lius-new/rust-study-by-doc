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


    println!("The area of the rectangle is {} square pixels.", area_v4(rectangle));
    println!("{:?}", rectangle); // value borrowed here after move
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
