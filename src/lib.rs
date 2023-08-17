// lib crate中声明和二进制crate中声明同样的,好像也不冲突,因为如果lib要使用二进制crate的mod好像不能直接使用,而是要自己也声明这个mod
// 通过在文件夹中创建mod.rs文件,感觉要比在外面添加mod name.rs文件要好得多. 比如外面如果有一个garden.rs,那么mod一多就会有很多这样的文件了
pub mod garden;

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}
