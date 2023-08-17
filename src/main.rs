// 在二进制的根crate中使用mod来创建
mod garden;

// 绝对路径: 以crate开头的全路径.对于外部crate代码以crate名开头的绝对路径. 对于当前crate的代码,则以字面值crate开头
// 实际上不管是外部crate还是当前crate好像都是可以以crate名开头.
use rust_32::front_of_house;// 这两者都是引入当前crate的代码,那么就是可以以crate开头
use crate::front_of_house;  // 这两者都是引入当前crate的代码,那么就是可以以crate开头

fn main() {
    let asparagus = garden::vegetables::Asparagus {};
    dbg!(asparagus);

    front_of_house::hosting::add_to_wait_list();
}