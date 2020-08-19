use std::{env, process};//导入env中的std::env::args()方法,能够读取命令行的输入
use std::fs;
use std::error::Error;
use minigrep::Config;
use minigrep::run;
//文件流读入文件
// Rust大型命令行程序的一般分离方式
// 1.使用参数值调用命令行解析逻辑
// 2.设置任何其他的配置
// 3.调用 lib.rs 中的 run 函数
// 4.如果 run 返回错误，则处理这个错误
fn main() {
    let args:Vec<String> = env::args().collect();
    let parse_conf = Config::new(&args).unwrap_or_else(|err|{
        println!("Problem parsing arguments:{}",err);
        process::exit(1);
    });//由于new的签名变更,因此parse处多出了错误处理代码
    //这里主要是显示错误信息以及让炸掉的程序退出
    //退出利用了std::process,从而让炸掉的程序快速退出,传递的数字为状态标识码
    println!("Searching for {}", parse_conf.query);
    println!("In file {}", parse_conf.filename);
    //处理run的错误
    if let Err(e) = run(parse_conf){
        println!("Application error:{}",e);
        process::exit(1);
    }
}

