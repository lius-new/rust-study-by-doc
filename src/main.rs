// 在二进制的根crate中使用mod来创建
mod garden;

fn main() {
    let asparagus = garden::vegetables::Asparagus {};
    dbg!(asparagus);
}