// 可以通过as来设置被use的新得名称
use std::io::Result as IoResult;

// lib crate中声明和二进制crate中声明同样的,好像也不冲突,因为如果lib要使用二进制crate的mod好像不能直接使用,而是要自己也声明这个mod
// 通过在文件夹中创建mod.rs文件,感觉要比在外面添加mod name.rs文件要好得多. 比如外面如果有一个garden.rs,那么mod一多就会有很多这样的文件了
pub mod garden;

fn deliver_order() {}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("world"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

use garden::vegetables::front_of_house;

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("bye");
    meal.toast;
    // meal.seasonal_fruit; // 此时无法访问,因为其并不是pub
    front_of_house::hosting::add_to_wait_list(); // 其可以直接访问到front_of_house,因为和其所在函数自傲同一scope
}

mod customer {
    pub fn eat_at_restaurant() {
        use crate::garden::vegetables::front_of_house;
        front_of_house::hosting::add_to_wait_list(); // 此时无法访问,front_of_house

        super::front_of_house::hosting::add_to_wait_list();
    }
}
