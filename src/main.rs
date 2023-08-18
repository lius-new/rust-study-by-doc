fn main() {
    // 字符串就是作为字节的集合外加一些方法实现的.
    // 事实上String被实现为一个带有一些额外保证限制功能的字节vector的封装
    let mut s = String::new();
    let data = "initial contents";
    let s = data.to_string();
    let mut s = "initial contents".to_string();

    // 添加字节
    s.push('o');
    // 添加其他字符串
    s.push_str("hello");

    // 创建两个字符串
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");

    // + 的函数签名,第一个参数会丢失所有权,第二个参数需要是一个引用
    // fn add(self, s: &str) -> String {
    let s3 = s1 + &s2;
    println!("{}", s1); // s1 的所有权已经转移到s3

    // 使用format来拼接字符串
    format!("{s1} - {s2}");
}