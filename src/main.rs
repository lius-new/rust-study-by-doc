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

impl<T, Y> Point<T, Y> {
    fn x(&self) -> T {
        &self.x
    }
}

impl Point<i32, i32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// 结构体定义的泛型类型参数并不总是与结构体方法签名中使用的泛型类型是同一类型
// 注意X,Y泛型与结构体对应
// X2,Y2泛型与函数签名对应
impl<X, Y> Point<X, Y> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
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