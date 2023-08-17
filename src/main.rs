fn main() {
    let width = 30;
    let height = 50;

    println!("The area of the rectangle is {} square pixels.", area(width, height))
}

fn area(width: i32, height: i32) -> i32 {
    width * height
}