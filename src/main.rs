use std::collections::{HashMap};

fn main() {

    // HashMap的使用.
    // HashMap将数据存储在堆上. HashMap的数据类型是同质的. 这表示key和value的类型可以是不一样的,但是其各自的类型必须是一样的
    let mut hash_map = HashMap::new();
    hash_map.insert(String::from("hello"), 0);
    hash_map.insert(String::from("world"), 0);

    // get方法获取的是Option<&i32>类型,通过copied来获取Option<i32>类型,接着调用unwrap_or且指定没有值的时候使用0
    let r = hash_map.get("world").copied().unwrap_or(0);

    for (key, value) in hash_map.iter() {
        println!("{key}:{value}")
    }

    let field_name = String::from("hello");
    let field_value = 2;

    // 如果是实现了copy trait类型的数据,那么值可以拷贝进入HashMap,如果是没有实现的,那么所有权就会发生转移, 此时HashMap的变量才是所有者
    // 注意如果保存的是引用值,那么就要保证引用值的声明周期在HashMap的生命周期范围内是有效的.
    hash_map.insert(field_name, field_value);


    // 插入一个值

    // 直接插入,不考虑覆盖的情况
    hash_map.insert("hello".to_string(), 0);

    // 考虑覆盖的情况
    // entry会检查对应的key是否存在,如果没有才插入值
    // 如果存在就返回这个值的可变引用
    let r = hash_map.entry("hello".to_string()).or_insert(0);

    let text = "hello world wonderful world";
    let mut hash_map_count = HashMap::new();
    for x in text.split_whitespace() {
        let count = hash_map_count.entry(x).or_insert(0);
        *count += 1
    }
    println!("{:#?}", hash_map_count)
}