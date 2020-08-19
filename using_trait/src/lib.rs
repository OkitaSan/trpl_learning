//trait是将方法签名组合起来的方法，定义实现一些目的所必须行为的集合
//用trait关键字声明trait，后跟trait名，大括号中声明对应的方法签名
//具体实现的话需要每一个实现对应trait类型的方法去实现
//编译器也会检查实现了这个trait的类型拥有完全一致的方法

//如果别人想要用你的这个crate中的Summary trait去实现自己的方法,需要将这个Trait导入到自己的作用域中
//并且这个导出给别人的trait也要是公开的
//注意:实现trait时需要满足trait本身或者要实现trait的类型位于crate的本地作用域
//从而防止某个用户给外部类型实现某个外部trait,避免冲突

use std::fmt::{Display, Debug};

//trait可以指定默认实现,这样的话可以选择是否重载默认实现
//如果trait中有方法已经有默认实现的话可以不用去重载,把别的没给默认实现的方法重载即可
//trait中默认实现可以允许调用trait中给出的其他方法,不管其他方法有没有实现
//不过没有办法从相同方法的重载实现中调用默认方法
pub trait Summary{
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String{
        format!("(Read more from {}...)",self.summarize_author())
    }
}
//在结构体中实现对应的trait
pub struct NewsArticle{
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String
}
//实现对应的trait方法
//imple XXtrait for XXstruct
impl Summary for NewsArticle{
    fn summarize_author(&self) -> String{format!("{}",self.author)}
    fn summarize(&self) -> String{
        format!("{} ,by {} ({})",self.headline,self.author,self.location)
    }
}
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
impl Summary for Tweet{
    fn summarize_author(&self) -> String{format!("{}",self.username)}
    fn summarize(&self) -> String{
        format!("{}:{}",self.username,self.content)
    }
}
pub struct WXPublicAccount{
    pub username: String,
    pub content:String
}
//如果不想重载默认实现的话直接给出空的impl块(如果impl块中全是默认方法)
//或者只实现impl块中没有默认实现的方法
impl Summary for WXPublicAccount{fn summarize_author(&self) -> String{format!("{}",self.username)}}
//trait可以作为参数
//在这里指定了所有实现Summary trait的方法都可以调用这个方法
pub fn notify(item:impl Summary){
    println!("Breaking news! {}",item.summarize());
}
//也可以写成trait bound的形式
//在单参数下是一样的,多参数会有所不同
//在T后面跟一个冒号写明对T类型需要实现哪些trait
pub fn notify_bound<T:Summary>(item:T){
    println!("Breaking news! {}",item.summarize());
}
//多个参数时，利用trait bound可以指定参数的类型都是同一类型
//而直接用impl trait的话参数是满足trait限制的函数即可
pub fn notify_mutiple<T:Summary>(item1:T,item2:T){
    println!("Breaking news! {} {}",item1.summarize(),item2.summarize());
}
pub fn notify_mutiple_variant(item1:impl Summary,item2:impl Summary){
    println!("Breaking news! {} {}",item1.summarize(),item2.summarize());
}
//通过+运算符可以实现多个trait的语义限制
//impl trait语法版本
pub fn notify_mutiple_by_mutiple_trait(item1:impl Summary + Display){}
//泛型trait bound版本
pub fn notify_mutiple_by_generic_trait_bound<T:Summary + Display>(item1:T){}
//通过where简化的trait bound版本
fn some_function_using_where<T,U>(x:T,y:U)
    where T:Display + Clone,U:Clone + Debug{}
fn notify_mutiple_using_where<T,U>(x:T,y:U)
    where T:Summary + Display,U:Clone + Summary + Display{}
//也可以指定返回某个trait的类型
//使用impl trait语法
fn returns_summrizable() -> impl Summary{
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
//但是实际返回的类型只能是单一的类型
//多个分支返回值静态类型必须完全一致，而不是实现返回值trait约束的不同类型
// fn returns_summarizable(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from("Penguins win the Stanley Cup Championship!"),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from("The Pittsburgh Penguins once again are the best
//             hockey team in the NHL."),
//         }
//     } else {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from("of course, as you probably already know, people"),
//             reply: false,
//             retweet: false,
//         }
//     }
// }
//利用trait bound可以有条件的实现方法
//需要在impl后面指定trait的约束
struct Point<T>{
    x: T,
    y: T
}
impl<T> Point<T>{
    fn new(x: T, y: T) -> Point<T>{
        Point { x, y }
    }
}
//利用trait为实现某些trait的类型实现方法
impl<T:Display + PartialOrd> Point<T>{
    fn cmp_and_displays(&self){
        if self.x >= self.y{
            println!("The largest member is x = {}", self.x);
        }else{
            println!("The largest member is y = {}", self.y);
        }
    }
}
//也可以给任何实现了特定trait类型有条件的实现trait
//比如可以为实现了Display trait的类型实现ToString trait
// impl<T: Display> ToString for T {
//     // --snip--
// }