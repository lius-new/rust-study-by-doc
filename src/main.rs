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
}