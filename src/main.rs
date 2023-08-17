// 在二进制的根crate中使用mod来创建
mod garden;
mod lib;

// 绝对路径: 以crate开头的全路径.对于外部crate代码以crate名开头的绝对路径. 对于当前crate的代码,则以字面值crate开头
use crate::lib::front_of_house;  // 事实上好像main 是二进制crate的根而lib是库crate的根,他们表示的crate并不是同一个意思.那么如果要使用lib crate,那么指定crate::lib::... , 感觉还不如直接crate名::mod名.

fn main() {
    let asparagus = garden::vegetables::Asparagus {};
    dbg!(asparagus);

    front_of_house::hosting::add_to_wait_list();
}