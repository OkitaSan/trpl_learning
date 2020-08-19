fn main() {
    //创建枚举的示例
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    //调用函数
    route(four);
    route(six);
    route(IpAddrKind::V4);
    println!("Hello, world!");
    let home = IpAddrStr::V4(String::from("127.0.0.1"));
    let loopback = IpAddrStr::V6(String::from("::1"));
    let m1 = Message::ChangeColor(1,2,3);
    let m2 = Message::Write(String::from("Hello, world"));
    let m3 = Message::Move{x:1,y:2};
    let m4 = Message::Quit;
    m1.show();m2.show();m3.show();m4.show();
    //Option枚举是一个泛型枚举,包含Some<T>和None两种，表达是否有值的语义
    let some_number = Some(5);
    let absent_number:Option<i32> = None;
    value_in_us_coin(UsCoin::Quarter(PartOfUsState::California));
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    //注意：所有的模式匹配都是有穷尽的
    //_表示通配符
    let some_u8value = 0;
    match some_u8value{
        1 => println!("One"),
        3 => println!("Three"),
        5 => println!("Five"),
        //下面的表示通配符
        _ => ()
    }
    let a_uscoin = UsCoin::Quarter(PartOfUsState::California);
    //if let控制流是对match的简化
    let some_u8value = Some(0u8);
    if let Some(value) = some_u8value{
        println!("{}",value);
    }else{
        println!("Not Zero");
    }
    a_uscoin.get_state();
}
//定义函数获取枚举
fn route(ip_type:IpAddrKind){}
//定义了一个枚举，有两个成员V4与V6
enum IpAddrKind{
    V4,
    V6
}
//枚举也可以把成员和指定的数据类型关连
enum IpAddrStr{
    V4(String),
    V6(String)
}
//枚举的不同成员关联的数据类型可以不同
enum IpAddr{
    V4(u8,u8,u8,u8),
    V6(String)
}
//此外，枚举可以实现方法
enum Message {
    Quit,
    Move { x: i32, y: i32 }/*Move包含一个匿名结构体*/,
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message{
    fn show(&self) {
        match self {
            Quit=> {println!("Quit");}
            Move => { println!("Move")}
            Write=>{println!("Write")}
            ChangeColor=> { println!("ChangeColor")}
        }
    }
}
enum Coin{
    Penny,Nickel,Dime,Quarter
}
fn value_in_cents(coin:Coin) -> u8{
    //利用match进行模式匹配获取对应的美元硬币的值
    //match的每一个分支，格式为 模式=>代码块
    //匹配的模式执行对应的代码块，代码块的结果即为返回值
    match coin{
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25
    }
}
//模式匹配也可绑定值的模式
#[derive(Debug)]
enum PartOfUsState{
    Alabama,Alaska,Arizona,Arkansas,California,Colorado,Connecticut,Washington,Illinois,Indiana,Iowa,Kansas,Kentucky,Louisiana
}
#[derive(Debug)]
enum UsCoin{
    Penny,
    Nickel,
    Dime,
    Quarter(PartOfUsState),
}
impl UsCoin{
    fn get_state(&self) -> Option<PartOfUsState>{
        match self{
            UsCoin::Quarter(UsState) => Some(UsState),
            _ => None
        }
    }
}
fn value_in_us_coin(coin: UsCoin) -> u8{
    match coin{
        UsCoin::Penny => 1,
        UsCoin::Nickel => 5,
        UsCoin::Dime => 10,
        UsCoin::Quarter(state) => {
            //利用模式匹配解构出了state的值
            println!("{:?}",state);
            25
        }
    }
}
fn plus_one(x:Option<i32>) -> Option<i32> {
    match x{
        Some(x) => Some(x+1),
        None => None
    }
}
