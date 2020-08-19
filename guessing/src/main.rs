/// rust程序本身已经在prelude模块里包含了部分模块，比如下面的stdio
use std::io;
/// 使用下面的thread_rng时要包含对应的trait
use rand::Rng;
/// 大小关系的比较时用到了
use std::cmp::Ordering;
fn main() {
    println!("Hello, world!");
    let secret_number = rand::thread_rng().gen_range(1,101);
    println!("the number is {}",secret_number);
    loop {
        /// 注意rust默认的let要是不加mut的话就是不变的
        let mut guess = String::new();
        /// 从标准输入中读入字符串传给guess(注意rust中的可变引用必须显式指明，包括在传参的时候)
    /// 获得一个io::result枚举，后面的expect函数如果枚举是Err的话直接打印错误信息
        io::stdin().read_line(&mut guess)
            .expect("Failed to read the line");
        println!("You guessed: {}", guess);
        /// rust虽然是静态类型语言，但是支持变量的shadowing，可以复用之前的变量的名字
        /// 下面是字符串到数字的转换，注意i字符串要去除空白字符，并且parse方法也是返回枚举的，且要人工指定解析数字的类型
        /// 利用模式匹配解构出parse的数字，如果正确就继续否则直接continue
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue
        };
        /// 利用match语句的模式匹配功能进行比较，有三个对应的枚举，对应路径执行对应代码
        /// 这里rust编译器会推断出secret_number类型为u32
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {println!("You win!");break;},
            Ordering::Greater => println!("Too big!")
        }
    }
}
