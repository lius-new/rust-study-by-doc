fn main() {
    // slice 允许引用一段连续的集合序列,而不是整合整个集合. 只是引用而非所有者自然没有所有权
    let mut s = String::from("hello world");
    let _r = first_word(&s); // 注意这个函数的返回值是与s不相关的一个值,在使用其获取的长度后两者就已经不相关了.

    s.clear(); // 此时s已经被清空了,但是后续访问长度其依然是原先的长度

    println!("s: [{s}]");

    // 使用b''的字面量其类型是u8,
    // 使用b""的字面量其类型是一个u8类型的数组
    let b = b' ';
    let world = b"world";
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { // 因为bytes是u8类型的数组的引用
            return i;
        }
    }
    s.len()
}