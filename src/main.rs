// 结构体, 类似元组类型可以存放任意数据类型的数据,但是必须为每一个字段命名
// 不需要为某个属性mut声明,因为这只是一个类型
struct User {
    name: String,
    // 使用String而非是&str,如果结构体往往会拥有它所有字段数据,为此整个结构体有效那么其数据也是有效的.
    // name: &String, 结构体不允许将某个字段设置为&,引用,因为实例要拥有其每个属性的的所有权.
    age: i32,
    email: String,
    // desc: &'static str, 这样为什么可以, 结构体可以包含其他值的引用,但是要确保其引用的数据是否还存在,此时便要用到声明周期
}

fn build_user(name: String, email: String) -> User {
    User {
        name,
        email,
        age: 10,
    }
}

struct Test {
    arr: [i32; 5],
}

// 元组结构体
struct Color(i32, i32, i32);

// 类单元结构体. 在使用类单元结构体的时候,其没有任何字段.这有些像static的工具类,但是事实上工具类也会有自身的属性
struct AlwaysEqual;

impl AlwaysEqual {
    fn test(self: &Self) -> Self {
        AlwaysEqual {}
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

    // 可以通过别的结构体实例来创建新得结构体的实例
    // 设置值的字段应有与给定实例对应字段相同的值
    let u2 = User {
        age: u.age,
        name: u.name,
        email: u.email,
    };

    // ..u2必须放在最后面,以指定其余应从u2的相同字段中获取其值.
    let u3 = User {
        ..u2
    };

    // 在使用这样的方式来创建新得实例时候实际上发生了值的移动,存储在堆中的数据会发生移动. 但是访问非堆中的数据依旧可以访问
    // println!("{}", u2.name)

    println!("{}", u2.age);

    // 元组和struct有着类似的地方,比如可以存储不同类型的数据,比如都是通过.来访问其中的数据

    let color = Color(1, 2, 3);
    let c = &color[..];
}
