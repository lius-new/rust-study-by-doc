// 函数命名使用小写,通过_进行单词的分割
fn main() {
    println!("Hello,world");
    another_function()

    // 注意函数由一系列语句和表达式组成. 且rust是一门基于表达式的语言
    // 语句是执行一些操作但是不返回值的指令
    // 表达式计算并产生值
    let x = 5; // 该行是一个语句并不返回值.

    // let y = (let x = 5); // let x = 5是一个表达式,其不会返回任何值,那么赋值会发生错误

    let x = y = 3; // 这同样无法对x进行赋值,因为y = 3是表达式,其结果是单元值

    // 常见的表达式有: 函数调用,宏调用,用大括号创建新得块作用域

    let h = {
        let x = 4;
        x
    };
}

// rust 不关心函数定义的前后,而关心的是函数所处的作用域
fn another_function() {
    println!("Another function.")
}

// 参数, 注意rust必须为参数指定数据类型
fn another_function_v2(x: i32) {
    println!("Another function: {}", x);
}