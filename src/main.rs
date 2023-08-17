fn main() {
    let width = 30;
    let height = 50;

    println!("The area of the rectangle is {} square pixels.", area(width, height));
    println!("The area of the rectangle is {} square pixels.", area_v2((height, width)));
    println!("The area of the rectangle is {} square pixels.", area_v3((height, width)));
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
