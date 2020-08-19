use std::collections::HashMap;
fn main() {
    //新建一个hashtable并向其中插入键值对
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"),10);
    scores.insert(String::from("Green"),20);
    //使用元组vector的collect方法
    //这里是构建了两个vector,然后把他们全部都变成迭代器再利用zip函数压缩成一个存放元组的vector
    //最后利用collect函数它转化成一个集合
    //注意需要写类型注解来确定到底是哪个集合
    //HashMap<_,_>表示可以推断hashmap的key和value
    let teams = vec!["Blue".to_string(), "Green".to_string()];
    let initial_scores = vec![10,50];
    let scores:HashMap<_,_> = teams.iter().zip(initial_scores.iter()).collect();
    //注意，如果insert类型没有实现copy trait的话insert时就会被移动
    let field_name = "favourite color".to_string();
    let field_value = "Blue".to_string();
    let mut map = HashMap::new();
    map.insert(field_name,field_value);
    //println!("{} {}",field_name,field_value);此时field_name,field_value两个变量都被move走
    //利用get方法可以访问hashmap中的值，用key来访问
    //返回一个Option<&V>
    let team_name = "Blue".to_string();
    let value = scores.get(&team_name);
    if let Some(&&value) = value {
        println!("{}",value);
    }
    //Hashmap也是可以遍历的
    for (&key,&&value) in &scores {
        println!("{} {}",key,value);
    }
    //insert插入的时候可以直接无脑覆盖先前的key
    let mut same_key = HashMap::new();
    same_key.insert(String::from("Blue"),10);
    same_key.insert(String::from("Blue"),20);
    println!("{:?}",same_key);
    //也可以只在没有这个键的时候插入
    //利用entry方法,它返回一个枚举,然后可以配合or_insert方法插入对应的值
    //Entry对应的or_insert方法在key对应value存在时返回value可变引用
    //否则插入对应的值，然后返回值可变引用
    same_key.entry("Blue".to_string()).or_insert(100);
    same_key.entry("Red".to_string()).or_insert(80);
    //示例:单词计数程序
    let mut word_cnt = HashMap::new();
    let text = "hello world wonderful world";
    for word in text.split_whitespace(){
        let cnt = word_cnt.entry(word).or_insert(0);
        *cnt += 1;
    }
    println!("{:?}", word_cnt);
    println!("Hello, world!");
}
