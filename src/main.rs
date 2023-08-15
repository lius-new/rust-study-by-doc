fn main() {
    // :: 表示这个函数是一个关联函数
    // String 和 str是不同的, 一个存储在堆中,需要开辟内存空间后存储且大小是可变的(这取决于开辟的内存空间的大小),而后者直接放入栈中大小是固定的.
    let mut s = String::from("this is test string");
    s.push_str(" hello");
    println!("{}", s);

    // String 这类存储在堆中的创建意味着:
    // 1. 要向运行时内存分配器请求内存
    // 2. 当处理完String时将内存返回给分配器的方法

    // 这好像所有权场景和配,不管是栈还是堆中的数据,内存在拥有它的变量离开作用域后就被自动释放(drop).

    // 注意5 和 x都是固定的的大小存储在栈中. 对于y会拷贝x的值放入栈顶, 而x会在y之前放入栈中
    let x = 5;
    let y = x;

    // s1 和 s2的机制并不是一样的. 它们会和内存分配器交互且存入堆中. 但是并不会发生简单的拷贝而已
    // String 由三部分组成(ptr:指针,len:长度,capacity:容量). 他们存储在栈中,而实际上的内容存储在堆中
    // 在将s2 = s1的过程中,将ptr,len,cap拷贝给了s2, 可存储在堆中的数据并未发生改变.
    // 事实上,如果两个指针都会尝试去和内存分配器沟通释放数据, 这样可能会出现二次释放的错误,安全漏洞.
    // 为了确保内存安全,在let s2 = s1的时候,rust便s1不再有效且在作用域结束也不会释放.
    let s1 = String::from("hello world");
    let s2 = s1;
    println!("{s2}");

    // 移动而非拷贝(浅深)
    // 并非浅拷贝,因为拷贝后前者被提前释放()
    // 并非深拷贝,因为只拷贝ptr,len,cap
    // 是移动
    // rust永远不会自动创建数据的深拷贝,因为rust认为任何自动复制都可以被认为对运行时性能影响较小.

    // 主动的拷贝,这会耗费更多的资源. 重新请求内存分配器寻址和开辟内存空间,最后还的记录后再返回
    let s3 = s2.clone();

    // 在栈上的拷贝
    // 栈中的数据不通过内存分配器来分配空间,存储的是大小固定且以知的大小的数据,这样的拷贝是快速的.且是创建新得变量保存一个新的值一样,而非移动,引用.

    // copy trait 和drop trait.
    // copy trait 往往作用在存储在栈中的数据. copy 同样也是往栈新放入数据. .
    // drop trait 往往处理与存储在堆中的数据. 在作用域结束的时候回收内存.
    // 放入栈中的数据在作用域离开后就移出栈而非drop

    let mut s4 = String::from("hello world");
    {
        let s5 = s4;
        s4 = s5;
    }
    println!("{s4}");

    let s6 = String::from("hello world");
    takes_ownership(s6); // s6 所有者被移入函数

    let i = 5;
    makes_copy(i); // i32 实现了copy trait, 值并不会发生移动而是拷贝
}

fn takes_ownership(s: String) {
    println!("{s}")
}

fn makes_copy(i: i32) {
    println!("{i}")
}
