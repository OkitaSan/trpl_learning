//Rust中String类型和字符串切片类型都是UTF-8编码
fn main() {
    //创建空字符串
    let mut empty_string = String::new();
    //利用&str的to_string方法获得String
    //只要实现Display trait即可
    let data = "initial_content";
    let mut string_from_str = data.to_string();
    let mut string_from_literal = "literal".to_string();
    //更新字符串内容
    //push_str接受slice参数,追加整个字符串
    string_from_literal.push_str("bar");
    //push接受单个字符,向尾部追加字符
    string_from_literal.push('b');
    println!("{}",string_from_literal);
    println!("Hello, world!");
    //拼接字符串
    //利用+运算符来拼接
    //注意由于add函数的签名为fn add(self,s:&str) -> String
    //因此lhs会丧失所有权，第二个就不会
    //第二个参数要注意签名匹配
    let s1 = "Hello".to_string();
    let s2 = " World!".to_string();
    //虽然&s2类型为&String,但是&String强转成&str
    //在函数调用的时候&s2被强转成&s2[..]
    let s3 = s1 + &s2;
    //较为复杂的字符串格式化,可以利用format!宏
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}",s1,s2,s3);
    println!("{}",s);
    //由于Rust字符串编码是UTF-8，因此不支持直接下标取得元素方法
    //String是vec<u8>封装
    let len = "蒸羊羔蒸熊掌烧花鸭……".to_string().len();
    println!("{}",len);
    //utf-8字符是多字节变长编码
    //但是字符串slice是会被允许的
    let hello_in_russia = "Здравствуйте".to_string();
    let s = &hello_in_russia[0..4];
    println!("{}",s);
    //不过要是正好slice到中间自负的话会直接panic
    //也可以遍历字符串
    //遍历字符串的每一个字符
    for c in "UTF-8编码真的好复杂QAQ".chars() {
        println!("{}",c);
    }
    //遍历构成字符串的每一个UTF-8字节
    for b in "UTF-8编码真的好复杂QAQ".bytes() {
        println!("{}",b);
    }
}
