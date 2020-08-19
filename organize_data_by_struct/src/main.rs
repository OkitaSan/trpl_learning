fn main() {
    println!("Hello, world!");
    //定义结构体变量方法
    //用key:value键值对表征
    let user1 = User{
        email:String::new(),
        username:String::new(),
        active:false,
        sign_in_count: 1
    };
    let mut user1 = User{
        email:String::new(),
        username:String::new(),
        active:false,
        sign_in_count: 1
    };
    user1.email = String::from("2333@2333.com");
    let user2 = User{
        email:String::new(),
        username:String::new(),
        active:user1.active,
        sign_in_count: user1.sign_in_count
    };
    //也可以采用语法糖形式,..user1表示剩下的字段值和user1一致
    let user2 = User{
        email:String::new(),
        username:String::new(),
        ..user1
    };
    //Rust中可以定义元组结构体,元组结构体的特点是具有强类型
    struct Point(i32,i32,i32);
    struct Color(i32,i32,i32);
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    //Color和Point结构体虽然都是三个i32构成元组，但是没法互转
    //除非定义生命周期，否则无法为结构体中定义引用成员
    let rect1 = Rectangle{height:100, width:100};
    println!("The area is {}",get_area(&rect1));
    //没实现Display trait的struct添加#[derive(Debug)]注解后下面的两种打印格式可用
    println!("{:?}",rect1);
    println!("{:#?}",rect1);
    println!("{}",rect1.area());
    println!("{}",rect1.area_and_give_out_ownership());//此时rect1已经无所有权
    //注意,rust语言在调用方法时会自动引用与解引用
    //也就是在调用object.somethings()时会自动添加& &mut *来让object与签名匹配
    let rect1 = Rectangle{height:100, width:100};
    let rect2 = Rectangle{height:200, width:100};
    println!("{}",rect1.can_hold(&rect2));
    let square1 = Rectangle::get_square(2);
    println!("{:#?}",square1);
}
//结构体字段直接用field_name:type表示
struct User{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}
fn build_user(email: String,username:String) -> User{
    //在函数最后直接构建并返回一个结构体
    //这里采用了语法糖形式,key:value如果key和value恰好同名，则直接用key的形式即可，相当于key:value
    //和下面语法是等价的
    User{
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}
//通过添加注解，派生Debug trait，以后可以直接打印调试信息
#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32
}
//下面实现了适用于Rectangle对象求面积的area方法
//除了静态方法以外，第一个参数为&self，表示对调用这个方法的对象的借用
//根据实际情况，可以决定是否借用self以及是否可变借用self
//Rust中的静态方法叫关联函数
//多个impl块是可以分散开来的
impl Rectangle{
    fn area(&self) -> u32{
        self.width * self.height
    }
    fn area_and_give_out_ownership(self) -> u32{
        self.width * self.height
    }
    fn can_hold(&self,other:&Rectangle) -> bool{
        self.width > other.width && other.height > self.height
    }
    fn get_square(edge:u32) -> Rectangle{
        Rectangle{height:edge,width:edge}
    }
}
fn get_area(rect: &Rectangle) -> u32{
    rect.width * rect.height
}