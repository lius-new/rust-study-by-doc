fn main() {
    // while 循环表达式
    let mut t = 5;
    while t > 1 {
        println!("{}", t);
        t -= 1;
    }

    let mut counter = 1;

    // while 表达式返回的总是(),单元值(单元元组), 因为其没有返回值, 在表达式为false就退出了
    let result = while counter < 20 {
        counter += 1;
        counter
    };
}
