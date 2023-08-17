// 定义一个枚举的类型
#[derive(Debug)]
enum IpAddrKind {
    V4,
    // 枚举的成员
    V6,
}

fn main() {
    let v4 = IpAddrKind::V4;
    println!("{:#?}", v4)
}