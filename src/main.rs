// 在二进制的根crate中使用mod来创建
// 事实上移动garden到lib crate,那么发现front_of_house也移动到lib crate中,也就是说mod在哪那么其内部的crate就以谁为父节点

// 绝对路径: 以crate开头的全路径.对于外部crate代码以crate名开头的绝对路径. 对于当前crate的代码,则以字面值crate开头
// 还是使用crate名::mod名这样的形式使用lib中的mod,因为使用crate::lib::modname报错了
// use rust_32::front_of_house;  // 事实上好像main 是二进制crate的根而lib是库crate的根,他们表示的crate并不是同一个意思.那么如果要使用lib crate,那么指定crate::lib::... , 感觉还不如直接crate名::mod名.

use rust_32::garden::{*};

fn main() {
    let asparagus = garden::vegetables::Asparagus {};
    dbg!(asparagus);

    vegetables::front_of_house::hosting::add_to_wait_list();
    front_of_house::hosting::add_to_wait_list();
}