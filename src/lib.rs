// lib crate中声明和二进制crate中声明同样的,好像也不冲突,因为如果lib要使用二进制crate的mod好像不能直接使用,而是要自己也声明这个mod
// 通过在文件夹中创建mod.rs文件,感觉要比在外面添加mod name.rs文件要好得多. 比如外面如果有一个garden.rs,那么mod一多就会有很多这样的文件了


pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_wait_list() {}

        fn seat_at_tablet() {}
    }

    mod saving {
        fn take_order() {}

        fn server_order() {}

        fn take_payment() {}
    }
}

// 这里好像就是使用crate::来从根引入.
// 而二进制crate好像是就使用crate名::来从根引入
pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_wait_list();

    // 相对路径
    front_of_house::hosting::add_to_wait_list();
}