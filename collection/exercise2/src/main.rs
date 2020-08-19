use std::io;
use std::collections::HashMap;
// 将字符串转换为 Pig Latin，也就是每一个单词的第一个辅音字母被移动到单词的结尾并增加 “ay”，所以 “first” 会变成 “irst-fay”。元音字母开头的单词则在结尾增加 “hay”（“apple” 会变成 “apple-hay”）。牢记 UTF-8 编码！
fn main() {
    let mut sequence_of_words = String::new();
    io::stdin().read_line(&mut sequence_of_words).expect("Reading Failed");
    let sequence_of_word:Vec<String> = sequence_of_words.split_whitespace().map(|x|{
        if &x[..1] == "a" || &x[..1] == "e" || &x[..1] == "i" || &x[..1] == "o" || &x[..1] == "u"{
            format!("{}-hay",x.to_string())
        }else{
            format!("{}-{}ay",&x[1..].to_string(),&x[..1])
        }
    }).collect();
    sequence_of_word.into_iter().for_each(|x|println!("{}",x));
    println!("Hello, world!");
}
