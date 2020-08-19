//panic!宏会直接打印出错误信息，展开然后清理栈的数据
//直接cargo run的话会提示对应的错误信息
// OkitaSan@DESKTOP-PMMH3QG  ~  projects  learningRust  panic_and_error 
//  master ≢ +24 ~0 -0 | +310 ~3 -0 !  cargo run
// Finished dev [unoptimized + debuginfo] target(s) in 0.00s
// Running `target\debug\panic_and_error.exe`
// thread 'main' panicked at 'Just panic!', src\main.rs:3:5 //显示panic提供的信息并且指明了源码中panic出现的位置
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
// error: process didn't exit successfully: `target\debug\panic_and_error.exe` (exit
// code: 101)
//默认不显示调用堆栈,除非把环境变量RUST_BACKTRACE设置成1
use std::io;use std::io::Read;use std::fs::File;use std::io::ErrorKind;
use std::error::Error;

//注意,?运算符返回错误只能在返回Result或者std::ops::Try类型的函数中进行
//可以修改函数返回值为Result<T,E>或者利用模式匹配来处理
//main也有两个有效的返回值,一个是()另外一个是Result<T,E>
//Box<dyn Error>被称为trait对象，它允许在main里使用?是能够返回任何类型的错误
fn main() -> Result<(),Box<dyn Error>> {
    //panic!("Just panic!");
    //let v = vec![1,2,3];
    //v[100];//Rust越界会直接panic他
    //backtrace在release版本时不会出现

    //File的open方法返回Result<File,io::Err>枚举，须自己解包
    let f = File::open("hello.txt");
    //如果ok返回句柄，否则直接panic报错
    // let f = match f{
    //     Ok(file) => file,
    //     Err(_) => panic!("File Reading Failed!")
    // };
    //我们也可以对文件处理造成的错误进行细化分析
    let f = match f {
        Ok(file) => file,
        //io::Error中有个返回io::ErrorKind枚举的kind方法
        Err(error) => match error.kind() {
            //匹配对应的错误类型
            //如果是没有文件的话尝试创建文件，然后再次匹配
            ErrorKind::NotFound => match File::create("hello.txt") {
                //成功直接返回文件
                Ok(fc) => fc,
                //否则panic
                Err(e) => panic!("Problem creating the file {:?}", e)
            }
            //其他错误类型直接panic
            other_error => panic!("Problem creating the file {:?}", other_error)
        }
    };
    //利用闭包改写的版本
    //unwrap_or_else方法,如果返回的Result枚举是Ok就直接返回，否则会按照提供的闭包来处理Err枚举
    let f = File::open("Cat.txt").unwrap_or_else(|err| {
        if err.kind() == ErrorKind::NotFound {
            File::create("Cat.txt").unwrap_or_else(|err| panic!("File creating failed:{:?}", err))
        } else { panic!("Problem Opening File:{:?}", err) }
    });
    //为了避免写个错误处理变成处理老太太裹脚布一样(x)，也有unwrap和expect函数可用
    //unwrap如果得到Err直接炸掉
    //expect也是直接炸掉，但是可以自定义炸掉时候的信息
    // let f = File::open("Makefile").unwrap();
    // let f = File::open("Makefile").expect("他炸了");



}//也可以控制如何处理错误
fn read_username_from_file() -> Result<String,io::Error> {
    let f = File::open("Hello.txt");
    //利用模式匹配结构枚举，如果有错误直接返回
    let mut file_handle = match f{
        Ok(content) => content,
        Err(error) => return Err(error)
    };
    let mut s = String::new();
    //匹配读文件的结果，成功返回Ok(content),失败返回Err(e)
    match file_handle.read_to_string(&mut s){
        Ok(content) => Ok(s),
        Err(error) => Err(error)
    }
}
//Rust提供了?运算符用来简化错误的传播
fn read_username_from_file_using_question_mark() -> Result<String,io::Error>{
    //这里的问号表达式表示成功返回对应值,失败直接返回包含错误信息的枚举
    let mut file = File::open("Hello.txt")?;
    let mut s = String::new();
    //下面的问号表达式中,只要错误实现了From trait中的from函数实现了对应的转换就可以把错误转换为指定的类型
    //然后提前返回函数,把错误传达给调用者
    //在这里转化成了返回值所规定的类型
    file.read_to_string(&mut s)?;
    Ok(s)
}
//？运算符也是可以允许链式方法调用的
fn read_username_from_file_using_question_mark_then_call_it_like_chain()
    -> Result<String,io::Error>{
    let mut s = String::new();
    //下面直接链式调用，功能和上面完全一致
    File::open("Hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}