use std::fmt::{Debug, Display, Formatter};
use std::iter::Sum;

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

// trait bound 语法(写法), 这样的写法和之前的写法是一样的。
pub fn notify_v2<T: Summary>(item: &T) {
    println!("breaking news:{}", item.summarize())
}

// 如果有多个泛型的参数呢?
pub fn notify_v3<T: Summary>(param1: &T, param2: &T) {
    println!("breaking news:{} / {}", param1.summarize(), param2.summarize())
}

// 如果一个函数以实现了两种trait类型(Summary,Display)的类型作为参数
pub fn notify_v4(param1: &impl Summary + Display) {
    println!("breaking news:{}  ", param1.summarize())
}

// 同样适应bound这样的写法
pub fn notify_v5<T: Summary + Display>(param1: &T) {
    println!("breaking news:{}  ", param1.summarize())
}

// 函数的参数是实现了两种trait的参数类型
pub fn some_function(t: &impl Display + Clone, u: &impl Clone + Debug) {
    todo!()
}

// 使用bound边界的写法(感觉看起来还是很啰嗦，但是感觉很明了)
pub fn some_function_v1<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) {
    todo!()
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