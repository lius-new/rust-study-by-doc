use std::fmt::{Display, Formatter};

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("hello world")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        todo!()
    }
}

impl Display for NewsArticle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Summary for dyn Display {
    fn summarize(&self) -> String {
        todo!()
    }
}


impl Summary for Vec<T> {
    fn summarize(&self) -> String {
        todo!()
    }
}

fn main() {
    // trait
    // 类型的行为是由类型的方法决定的。如果不同的类型调用相同的方法，那么这些类型就可以共享相同的方法了。
}