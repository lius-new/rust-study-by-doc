// 通过use来导入crate.
// rust中设定包含自动导入到每个程序的作用域的标准库的内容.这样的内容被称为预导入内容.
// 未包含在预导入的内容需要自己手动导入
// TODO: (chapter-1) 1. 那么我该将自己的内容放入预导入的内容中呢?
use std::io;

fn main() {
    println!("请输入你猜测的数字: ");

    // 1. 默认变量不可变,但是可以通过mut关键字来声明可变变化
    // 2. String,字符串类型,可增长的文本块
    // 3. new 属于关联函数(静态方法)
    // 4. 总之,下面创建了一个可变的字符串变量
    let mut guess = String::new();

    // io::stdio() 代表终端标准输入句柄的类型
    // read_line() 从标准输入句获取输入并追加到guess ,& 表示引用,其允许程序中多处访问同一个变量而无需在内存中拷贝变量
    // read_line() 返回result类型. 这是一个枚举类型,它包含两个实例(Ok,Err).  result实例拥有.expect方法.  当实例为Err的时候expect才会生效,此时程序会崩溃并在标准输出终端打印内容
    io::stdin().read_line(&mut guess).expect("输入的内容错误");

    println!("你输入了: {}", guess);
}
