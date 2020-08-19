fn main() {
    /// 注意let关键字对应的是**不可变变量**,之后运行时不能再赋值
    let x = 5;
    println!("x:{}",x);
    println!("x:{}",x);
    /// let mut就是**可变变量**，可以再赋值
    let mut y = 5;
    y = 6;
    /// rust中的const指的是常量，和C++的constexpr类似,声明时需要标注好类型,不能对其使用mut
    const CONSTANT_EXPRESSION:u32 = 13290;
    /// rust中支持对变量的shadowing,本质就是利用这个名字重建了一个新变量,shadowing前后的变量值和类型不必一致
    let z = 3;
    let z = z + 1;
    let z = z * 2;
    /// 注意，rust是静态类型语言
    /// 字符串解析数字需要加上类型注解,否则报错
    let parse_number:u32 = "42".parse().expect("Not a number!");
    ///标量类型
    ///整型包括有符号和无符号,具体为 i8-i128,isize与u8-u128，u_size
    ///有符号规定以补码存储
    ///isize/usize取决去计算机的具体架构，64位机为64位
    ///rust中的数字字面值允许使用类型后缀(12u8,256i128)，且允许使用_作为分隔符
    let decimal = 99_999;
    let i128decimal = 99_99i128;
    let hex = 0xff;
    let oct = 0o77;
    let bin = 0b1111_1111_1111_1111;
    ///注意byte前缀也是字面值，是单字节字符，不过仅限u8类型
    let byte = b'a';
    ///rust语言中，debug模式会报关于整数溢出的错误，但是release版本就不会，对应的值为溢出的数字和2^n取模的结果
    /// rust中，浮点数分f32与f64，均采用IEEE754表示法
    let float:f32 = 3.0;
    let double = 3.0;
    ///let computation = 3i32 + 3u32; //错误，不同类型之间数值无法进行隐式类型转换
    ///rust中bool值和算术运算不再赘述,需要注意的是rust数值之间不存在隐式类型转换。
    ///rust中char类型也是用单引号表示，但是注意其编码为utf-8
    let c = 'c';
    let chinese_character = '中';
    let emoji = '❤';
    ///rust有原生的复合类型，元组和数组
    ///rust中元组每个位置元素类型不必相同，类型注解是可选的
    let tuple1 = (1,9i128,9u32,'😂');
    let tuple2:(i32,i128,u32,char) = (1,9i128,9u32,'😂');
    ///元组中的元素可以通过模式匹配进行解构
    let (elem1,elem2,elem3,elem4) = tuple1;
    println!("{} {} {} {}",elem1,elem2,elem3,elem4);
    ///此外，也可以直接对元素进行访问
    let elem1 = tuple1.0;
    let elem2 = tuple1.1;
    let elem3 = tuple1.2;
    let elem4 = tuple1.3;
    println!("{} {} {} {}",elem1,elem2,elem3,elem4);
    ///和元组不同，数组中元素类型必须相同
    ///rust中内置数组长度是锁死的
    let array = [1,2,3,4,5];
    ///rust中可以按照[type;size]的语法给数组类型的元组做类型注解,但是必须把所有元素都写出来
    let array:[u32;6] = [1,2,3,4,5,6];
    ///也可以利用[initial_value;size]的语法直接初始化一个数组变量
    let array = [3;5];
    println!("{} {} {} {} {}",array[0],array[1],array[2],array[3],array[4]);
    ///注意rust的数组有运行时越界检查
    /// let out_of_range = array[12];
    println!("Hello, world!");
    another_function();
}
/// Rust中函数变量命名是snake_case，同时函数调用和函数实际定义的前后没有约束关系
/// 不需要像C/C++那样搞前置声明
fn another_function(){
    println!("Called from another function\n");
}
