use std::collections;
fn main() {
    //创建一个vector，指定存放i32元素
    let v:Vec<i32> = Vec::new();
    //利用宏来直接创建，此时注解不强制
    let v_from_marco = vec![1,2,3];
    //向尾部追加元素
    let mut v = Vec::new();
    //此时会根据添加的元素判断类型
    v.push(1);
    //vector遵循RAII模式
    {
        let v_in_scope = vec![1,2,3,4,5,6,7];
    }//v_in_scope的元素此时全部被丢弃
    //可以通过get方法或者是用索引访问vec中的项
    //get方法返回的是一个Option<&T>的枚举
    let third_ref = &v_from_marco[2];
    let third_from_get = v_from_marco.get(2);
    //下标索引越界会直接panic
    //let vec_out_of_range = &v[200];
    //get方法直接返回None枚举
    match v_from_marco.get(100){
        Some(value) => {
            println!("Element {} found!",value)
        },
        None => println!("Element not found!")
    }
    match v_from_marco.get(2){
        Some(value) => {
            println!("Element {:?} found!",value)
        },
        None => println!("Element not found!")
    }
    //Rust的借用检查器会检查引用是否有效,规则参见所有权规则
    let mut v_ownership_rule = vec![1,2,3];
    let first_immu = &mut v_ownership_rule[0];
    v_ownership_rule.push(4);
    //println!("{}",first_immu);这句话使用了前面的不变借用，而push由于可能的扩容问题导致第一个引用失效
    //对vector元素的遍历
    let mut v_visit = vec![1,2,3,4,5,6];
    //用for循环获取不变引用
    for elem in &v_visit {
        println!("{}",elem);
    }
    //也可以通过for循环利用可变引用改变vec的元素
    //注意改变元素时需要解引用
    for elem in &mut v_visit {
        *elem += 50;
    }
    for elem in v_visit{
        println!("{}",elem);
    }
    println!("Hello, world!");
}
