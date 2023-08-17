#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// impl Rectangle 块表示是结构体的上下文,表示里面的所有的方法都与结构体相关联
// 下面在Rectangle 结构体的上下文中定义一个函数,这个函数就是方法,这个方法的定义和普通函数差别在于可以直接使用&self,表示其为结构体实例的方法
// 注意: 在impl块中, Self表示impl块类型的别名
impl Rectangle {
    // 当然也可以写成  fn area(self: &Rectangle) -> u32 {
    // 当然也可以写成  fn area(self: &Self) -> u32 {
    // 注意: &self只是&Self的一个缩写
    fn area(&self) -> u32 {
        // fn area(&self) -> u32 {
        self.width * self.width
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area of the rectangle is {} square pixels.", rect1.area())
}