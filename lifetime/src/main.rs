use std::fmt::Display;
//Rust 中的每一个引用都有其 生命周期（lifetime），也就是引用保持有效的作用域
fn main() {
    //Rust的借用检查器
    // {
    //     let r;                // ---------+-- 'a
    //     //          |
    //     {                     //          |
    //         let x = 5;        // -+-- 'b  |
    //         r = &x;           //  |       |
    //     }                     // -+       |
    //     //          |
    //     println!("r: {}", r); //          |
    // }                         // ---------+
    //这里编译器做出了分析,因为r的生命周期是a,x的生命周期是b,a > b,因此产生悬垂引用,编译失败
    {
        let x = 5; //-----------------------------------'b
        let r = &x; //------------'a                   |
        println!("r: {}", r); //         |                   |
    }                        //----------+-------------------+
    //从上面可以看出,b的生命周期比a长,因此不会产生悬垂引用
    //使用带有生命周期函数的版本
    let string1 = "long string is very very very long".to_string();
    {
        let string2 = "I am not that long".to_string();
        let result = longest(string1.as_str(),string2.as_str());
        println!("{}",result);
    }//上面的例子中,取了string2的生命周期,result引用了到内部作用域结束都有效的值
    let string1 = "long string is very very very long".to_string();
    let result1;
    {
        let string2 = "I am not that long".to_string();
        let result = longest(string1.as_str(),string2.as_str());
        result1 = result;
    }
    //println!("{}",result1);//编译失败,因为这种调用使得返回值对应引用的生命周期大于标注的生命周期

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt{part:first_sentence};
    println!("{:?}",i);

    //有一种特殊的生命周期,'static，表示其能够存活在整个程序期间
    //字符串字面值默认拥有'static生命周期
    let s:&'static str = "fdsa";
}
//下面的函数无法编译通过,因为编译器不知道返回值是取哪个参数的生命周期
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }
// 因此需要加上生命周期注解
// //生命周期注解并不改变具体的生命周期的长短,只是描述了多个引用生命周期的关系
// //下面分别声明了两个生命周期,&'a i32表示不可变的带有生命周期注解的引用
// //&'a mut i32表示可变的带有生命周期注解的引用
// //生命周期的重点在于是让参数的生命周期是有关系的，从而方便编译器推断
// fn lifetime_annotation<'a>(refer:&i32,ref_with_annotation:&'a i32,mut_ref_with_annotation:&'a mut i32){
//
// }

//加了生命周期注解的版本
//注意和泛型参数一样,泛型生命周期参数需要声明在函数名和参数列表间的尖括号中
//函数调用时,被'a所替代的生命周期取x和y中生命周期小的那个
//下面生命周期注解的实际含义是 longest 函数返回的引用的生命周期与传入该函数的引用的生命周期的较小者一致。
fn longest<'a>(x:&'a str,y:&'a str) -> &'a str{
    if x.len() > y.len(){
        x
    }else{
        y
    }
}
//指定生命周期的正确方式和具体功能有关
//因为下面函数总是返回第一个参数,因此不需要给y指定生命周期
fn return_first<'a>(first:&'a str,second:&str) -> &'a str{
    first
}
//如果返回值没有指向参数的话,那么只可能是一个函数内部的值,因此会成为一个悬垂引用
// fn longest<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str()
// }

//结构体中也可定义生命周期
//和泛型参数一样,需要在结构体名字后声明生命周期参数
//这里表示该结构体不能比它part字段的引用存在的更久
#[derive(Debug)]
struct ImportantExcerpt<'a>{
    part:&'a str
}

//生命周期拥有省略规则
//规则1：每一个引用的参数都有自己的生命周期参数
//规则2：如果只有一个输入引用参数,那么它被予所有输出生命周期参数
//规则3：如果有多个参数,其中有一个参数是&self或&mut self，那么所有输出生命周期参数被赋予self的生命周期

//方法中的生命周期
//实现方法时结构体字段的生命周期必须总在impl后声明并在结构体名称后被使用
impl<'a> ImportantExcerpt<'a>{
    fn level(&self) -> i32 {3}
    fn announce_and_return(&self,announcement:&str) -> &str{
        //注意这里的返回值采用的是&self的生命周期
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn longest_with_announcement<'a,T>(x:&'a str,y:&'a str,ann:T) -> &'a str
    where T:Display{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
