fn main() {
    // 标量数据类型
    // 每个数据都有数据类型
    // rust是静态类型语言,在编译时就必须知道所有变量的类型

    // 无符号整数类型
    let x: u8 = 10;
    let x: u16 = 10;
    let x: u32 = 10;
    let x: u64 = 10;
    let x: u128 = 10;

    // 有符号整数类型
    let x: i8 = 10;
    let x: i16 = 10;
    let x: i32 = 10;
    let x = 10; // 默认是i32
    let x: i64 = 10;
    let x: i128 = 10;

    // 表示索引或者size的类型
    let x: isize = 10;
    let x: usize = 10;

    // 浮点类型
    let d = 3.14159261; // 默认是f64
    let d: f32 = 3.14159261;

    // 布尔类型
    let t = true;
    let f = false;

    // 字符类型
    // 注意使用单引号来声明字符类型,通过双引号来声明字符串字面量
    let c = 'z';
    let z: char = 'Z';
    let _heart_eyed_cat = '😻';
}
