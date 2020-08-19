//把Hashmap导出到作用域中
use std::collections::HashMap;
//导入两个模块
use std::fmt;
use std::io;
//设置导入的别名避免冲突
use std::fmt::Result;
use std::io::Result as IOResult;
//把rand包导入到当前作用域中
use rand;
//可以使用简化语法简化导入相同模块的不同项
use std::{cmp::Ordering,sync};
//和下面是等价的
//use std::cmp::Ordering;use std::sync;
//use std::io::{self,Write}和use std::io;use std::io::Write;效果是等价的
//下面的是把所有公有项导出到当前作用域
use std::collections::*;
//模块可以把一个crate中的代码进行分组
//模块也可以控制项的私有性
//本模块的模块结构如下
//注意crate根文件(src/lib.rs src/main.rs),他们的内容分别在crate模块结构的根组成了一个名为crate的模块
// crate
// └── front_of_house
//      ├── hosting
//      │   ├── add_to_waitlist
//      │   └── seat_at_table
//      └── serving
//          ├── take_order
//          ├── serve_order
//      └   ── take_payment
fn serve_order(){}
mod front_of_house{
    pub mod hosting{
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }
    fn fix_incorrect_order() {
        cook_order();
        //super是指向的父模块
        super::serve_order();
    }
    fn cook_order() {}
    mod serving{
        fn take_order() {}

        fn server_order() {}

        fn take_payment() {}
    }

}
fn function1() -> fmt::Result{}
fn function2() -> io::Result<()>{}
fn function3() -> Result{} //fmt模块下
fn function4() -> IOResult<()>{} //IO模块下
//可以定义公有的结构体和枚举，结构体的字段默认是私有的
//枚举的字段默认是公有的
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
    pub enum Appetizer {
        Soup,Salad
    }
}
//模块系统可以控制私有公有性
//Rust中默认所有项都是私有的,父不能调用子,子可以使用父
//在模块前加pub关键字可以让模块公有,在这个例子中,如果能够访问到front_of_house模块,hosting前加pub后也能访问到hosting模块
//但是模块公有不代表模块里面的内容是公有的,模块上的pub只允许它父模块引用他，模块中的内部要公有也需要加pub
//定义在同一模块中的同级别兄弟内容实可以互相直接访问的


//把绝对路径导出到作用域中,现在hosting就是有效的名称了
use crate::front_of_house::hosting;
//利用相对路径把结构体导出到作用域中
use back_of_house::Appetizer;
//pub use把私有名称公开重导入到作用域中,这样外部代码可以直接访问这个私有名称
//这里把add_to_waitlist做公开导出
pub use front_of_house::hosting::add_to_waitlist;
pub fn eat_at_restaurant(){
    //通过绝对路径方式调用
    crate::front_of_house::hosting::add_to_waitlist();
    //虽然front_of_house模块前面没加pub，但是eat_at_restaurant和front_of_house是同级的兄弟
    //因此可以访问,不用加pub
    //通过相对路径方式调用
    front_of_house::hosting::add_to_waitlist();
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("toast");
    //meal.seasonal_fruit = String::from("Apple");会报错,因为seasonal_fruit字段是私有的
    //使用模块中的枚举字段
    let order1 = back_of_house::Appetizer::Salad;
    let order1 = back_of_house::Appetizer::Soup;
    //通过导出到作用域后的名字调用
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    //利用导出的枚举名来进行调用
    //避免调用多个库后导出名称的二义性
    let mut order2 = Appetizer::Soup;
    //使用pub use重导出后的名字
    add_to_waitlist();
}
