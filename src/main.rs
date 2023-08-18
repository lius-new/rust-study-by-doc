fn main() {
    // 字符串就是作为字节的集合外加一些方法实现的.
    // 事实上String被实现为一个带有一些额外保证限制功能的字节vector的封装
    let mut s = String::new();
    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contents".to_string();
}