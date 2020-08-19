#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
pub fn add_two(a:i32) -> i32 { a + 2 }
pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }
    pub fn new_mutliple_panic(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}.",
                   value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}.",
                   value);
        }

        Guess {
            value
        }
    }
}
//Rust默认测试并行运行
//因此每个测试之间状态应该是独立的
//并且测试也会截获到stdout的内容,因此println!打印的东西就不行,需要cargo test -- --nocapture
//默认测试是全跑的,可以向cargo test传递测试名
//可以是具体测试名也可以是部分名,部分名会执行其匹配的所有测试
//可以在某些测试前加[ignore]注解忽略这个测试
//需要执行被忽略的测试的话就执行cargo test -- --ignored
#[cfg(test)]
mod tests {
    use super::*; //把父模块的名字导入到测试模块的作用域中
    #[test]//此注解表明下面的函数是测试函数
    #[ignore]
    fn it_works() {
        assert_eq!(2 + 2, 4);//使用assert_eq!宏检查测试结果
    }
    // #[test] //一个失败的测试
    // fn it_fails() {
    //     assert_eq!(1, 2);
    // }
    #[test]
    fn larger_can_holder_smaller(){
        let larger = Rectangle{width:8, height:8};
        let smaller = Rectangle{width:5, height:1};
        assert!(larger.can_hold(&smaller)); //assert!宏,检查是否为true
    }
    #[test]
    fn smaller_cannot_hold_larger(){
        let larger = Rectangle{width:8, height:7};
        let smaller = Rectangle{width:5,height:1};
        assert!(!smaller.can_hold(&larger));
    }
    #[test]
    fn it_adds_two(){
        assert_eq!(4,add_two(2));//assert_eq宏在测试的时候会打印lhs和rhs的值
        //因此lhs和rhs须实现PartialEq和Debug trait
        //但是这两个注解都是可派生的,直接#[derive(Debug,PartialEq)]加上去就行
    }
    #[test]
    fn it_isnot_four(){
        assert_ne!(13,add_two(2));//assert_
    }
    //assert系列宏后面也可追加对应的format!系列格式的参数,用于定制化出现的错误信息
    #[test]
    fn greeting_contains_name(){
        let result = greeting("Carol");
        assert!(result.contains("Carol")
                ,"Greeting did not contain name ,value was '{}'",result);
    }
    //测试是否panic
    #[test]
    #[should_panic]
    fn greater_than_100(){
        Guess::new(200);
    }
    //测试结果是否是对应的panic
    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100_new_mutilple(){
        Guess::new_mutliple_panic(200);
    }
    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100_new_mutilple_fails(){
        Guess::new_mutliple_panic(1000);
    }
    //测试Result<T,E>
    #[test]
    fn it_just_works() -> Result<(),String>{
        if 2 + 2 == 4{
            Ok(())
        }else{
            Err(String::from("Two plus two does not equal four"))
        }
    }
    #[test]
    fn show_stdout(){
        println!("{}","fdsafdsasdfa");
    }
}
