// 函数命名使用小写,通过_进行单词的分割
fn main() {
    println!("Hello,world");
    another_function()
}

// rust 不关心函数定义的前后,而关心的是函数所处的作用域
fn another_function() {
    println!("Another function.")
}

// 参数, 注意rust必须为参数指定数据类型
fn another_function_v2(x: i32) {
    println!("Another function: {}", x);
}