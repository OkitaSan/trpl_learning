//在这里声明一个模块,具体实现的话在src/front_of_house.rs文件中
mod front_of_house;
pub use crate::front_of_house::hosting;
pub fn eat_at_restaurant(){
    hosting::add_to_waitlist();
}