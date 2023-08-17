// 定义一个枚举的类型
#[derive(Debug)]
enum IpAddrKind {
    V4,
    // 枚举的成员
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: &'static str,
}


fn main() {
    let ipaddr1 = IpAddr {
        kind: IpAddrKind::V4,
        address: "localhost",
    };
    let ipaddr2 = IpAddr {
        kind: IpAddrKind::V6,
        address: "::127.0.0.1",
    };
}