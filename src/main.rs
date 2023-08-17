#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[derive(Debug)]
struct Rectangle2 {
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

    // 通过传入一个self,也就是将所有者传入进来,然后将其转换为另一种类型
    fn to_struct(self) -> Rectangle2 {
        Rectangle2 {
            width: self.width,
            height: self.height,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area of the rectangle is {} square pixels.", rect1.area());
    // let rect2 = rect1.to_struct();
    // println!("{}", rect1.width); // value的值发生了移动,已经无法访问, 注意这里并不是值发生了移动,而是在rect1.to_struct();过程中将rect1所有者传给了函数self参数,所有者移动了.

    let width = rect1.width; // 将值赋值给基本的数据类型好像不会发生值的移动关系
    println!("{}", rect1.width);

    let mut rect2 = Rectangle {
        width: 30,
        height: 50,
    };
    rect2.width = rect1.width; // 结构体之间的赋值并不会导致字段的所有者发生改变
    println!("{}", rect1.width);

    // 在c语言中,指针的操作通常要先获取到 object->something() 或 (*object).something().
    // 在rust中并没有等效的运算符, 但是rust采用自动引用和解引用, 也就是说访问方法的时候如.p1.distance(&p2)会自动转换为(&p1).distance(&p2);
}