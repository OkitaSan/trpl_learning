use std::env;//导入env中的std::env::args()方法,能够读取命令行的输入
use std::fs;//文件流读入文件
fn main() {
    let args:Vec<String> = env::args().collect();
    let parse_conf = parse_config(&args[..]);
    println!("{},{}",parse_conf.query,parse_conf.filename);
    let contents = fs::read_to_string("Poem.txt".to_string()).
        expect("Something went wrong reading the file");
    println!("{}", contents);
}
struct Config<'a> {
    query: &'a str,
    filename: &'a str
}
/// 这里采用了生命周期注解,看起来应该没事(x)
fn parse_config(args:&[String]) -> Config{
    let query = args[1].as_str();
    let filename = args[2].as_str();
    Config{query,filename}
}
