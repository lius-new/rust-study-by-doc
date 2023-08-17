#[derive(Debug)]
enum Message {
    Quit,
    // 未关联任何数据类型
    Move { x: i32, y: i32 },
    // 类似结构体的命名字段
    Write(String),
    //包含String
    ChangeColor(i32, i32, i32), // 包含3个i32
}

// 为枚举定义对不同实例的同样的处理方法
impl Message {
    fn call(&self) {
        dbg!(&self);
    }
}

fn main() {
    Message::Quit.call();
    Message::Move { x: 4, y: 5 }.call();
    Message::Write(String::from("hello world")).call();
    Message::ChangeColor(1, 2, 3).call();
}