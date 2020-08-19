use std::io;
use std::collections::HashMap;
// 给定一系列数字，使用 vector 并返回这个列表的平均数（mean, average）、中位数（排列数组后位于中间的值）和众数（mode，出现次数最多的值；这里哈希函数会很有帮助）。
fn main() {
    let mut line_of_number = String::new();
    match io::stdin().read_line(&mut line_of_number){
        Ok(size_of_line) => println!("Reading success!"),
        Err(_) => println!("Could not read line")
    };
    let mut vector_of_numbers:Vec<i32> = line_of_number.split_whitespace().map(|x:&str| -> i32{
        let num:i32 = x.parse().expect("Not a number!");num
    }).into_iter().collect();
    let mut word_cnt:HashMap<i32,usize> = HashMap::new();
    for elem in &mut vector_of_numbers {
        let elem_key = elem.clone();
        let times = word_cnt.entry(elem_key).or_insert(0);
        *times += 1;
    }
    let mut max_key = 0;let mut max_value:usize = 0;
    for (&key,&value) in &word_cnt{
        if value > max_value{
            max_value = value;
            max_key = key;
        }
    }
    println!("The word that appears the most frequent is {} , which appears {} times",max_key,max_value);
    let mut sum = 0;
    for &elem in &vector_of_numbers{
        sum += elem;
    }
    println!("The average of the number sequence is {}", (sum/(vector_of_numbers.len() as i32)) as f32);
    vector_of_numbers.sort();
    println!("{:?}",vector_of_numbers);
    println!("The mid of the number sequence is {}",vector_of_numbers[vector_of_numbers.len()/2usize]);
}
