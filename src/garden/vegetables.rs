#[derive(Debug)]
pub struct Asparagus {}

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
    crate::garden::vegetables::front_of_house::hosting::add_to_wait_list();

    // 相对路径
    front_of_house::hosting::add_to_wait_list();
}
