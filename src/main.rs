use std::fmt::{Display, Formatter};

pub trait Summary {
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
    fn summarize_author(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        todo!("hell")
    }
}

impl Display for NewsArticle {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Summary for dyn Display {
    fn summarize_author(&self) -> String {
        todo!()
    }
}

impl<T> Summary for Vec<T> {
    fn summarize_author(&self) -> String {
        todo!()
    }
}

pub fn notify(item: &impl Summary) {
    println!("breaking news:{}", item.summarize())
}

fn main() {
    // trait
    // 类型的行为是由类型的方法决定的。如果不同的类型调用相同的方法，那么这些类型就可以共享相同的方法了。
    let news = NewsArticle {
        author: String::from("hello world"),
        location: String::from("hello world"),
        content: String::from("hello world"),
        headline: String::from("hello world"),
    };
    notify(&news);
}