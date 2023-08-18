fn main() {
    // vector 一个集合类型,允许存储相同数据类型的值,其在内存中是相邻排列的. 其优势在于存储的值是可以改变的.

    // 默认情况下vsc必须要指定类型,因为vec的类型注解是泛型,默认在不插入值的情况下rust无法推断值是什么类型的.
    // let v = Vec::new();

    // 通过vec!宏来创建拥有值的vector
    let mut v = vec![1, 2, 3];

    // push , 注意vector必须是mut的.不然不允许修改
    v.push(4);
}