use std::{error::Error,fs};
//Box<dyn Error>表示接受任何类型的错误(Trait对象)
pub fn run(config:Config) -> Result<(),Box<dyn Error>>{
    let contents = fs::read_to_string(config.filename)?;
    println!("With text:\n{}",contents);
    Ok(())
}
pub struct Config<'a> {
    pub query: &'a String,
    pub filename: &'a String
}
/// 这里采用了生命周期注解,看起来应该没事(x)
/// 大胆一点,上&String
/// panic还是有一堆没用的信息,针对用户应该改成result
impl<'a> Config<'a>{
    pub fn new(args:&[String]) -> Result<Config,&'static str>{
        if args.len() < 3{
            return Err("Not enough arguments");
        }
        let query = &args[1];//使用生命周期注解的效果待查看
        let filename = &args[2];//使用生命周期注解的效果待查看
        Ok(Config{query,filename})
    }
}