use std::env::set_current_dir;

fn main() {
    let mut s = String::from("hello world");
    let s1 = &s[0..5];
    let s2 = &s[0..10];
    println!("{s1},{s2}");

    // 获取String所有值的切片
    let s3 = &s[..];
    // 获取从0..4值的切片
    let s4 = &s[..4];
    // 获取0..最后一个值的切片
    let s5 = &s[1..];

    let arr = [1, 2, 3, 4, 5];
    // 获取数组的切片
    let a1 = &arr[1..2];
    // 获取元组的切片
    let tumple = (1, 2, 3, );
    // 获取从1..到最后的切片.
    // let tumple1 = &tumple[1..];

    let s6 = second_string(&s);
    println!("{s6}");
    s.clear();
    // 如果后面依然有s6在借用,那么就不可以修改, 不可以修改,修改通常会使用的是可变引用,但是切片确实属于不可变引用,
    // 注意: &是引用的关系,那么引用就会安发生借用关系.必须要当s6归还后才可以访问到自己本身的变量

    // println!("{s6}");

    let s7 = "hello world";

}

fn second_string(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[i..];
        }
    }
    &s[..]
}

