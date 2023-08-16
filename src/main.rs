// 结构体, 类似元组类型可以存放任意数据类型的数据,但是必须为每一个字段命名
// 不需要为某个属性mut声明,因为这只是一个类型
struct User {
    name: String,
    age: i32,
    email: String,
}

fn build_user(name: String, email: String) -> User {
    User {
        name,
        email,
        age: 10,
    }
}

fn main() {
    // 实例
    let u = User {
        name: String::from("张三"),
        age: 10,
        email: String::from("email&email.com"),
    };
    // 用过.运算符来访问结构体中的数据,这与元组好像也是一样的
    println!("{}", u.email);

    // 移动原先的值
    let mut u = u;

    // 如果要改变值,那么这个实例就应该是可变的,且所有的值都是可变的
    u.email = String::from("hello world");
}
