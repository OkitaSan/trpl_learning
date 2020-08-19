fn main() {
    println!("Hello, world!");
    //参照C++的作用域规则，这里不再赘述
    {
        let s = "Hello";
    }
    //和字符串字面量不同，分配在堆上的字符串可以存储编译时大小位置的文本
    //用字符串字面量构建文本
    let s = String::from("Hello");
    let mut s = s;
    //可以修改字符串文本，push_str向后追加
    s.push_str(",World!");
    //Rust对堆上持有内存的对象采用RAII策略
    {
        let raii = String::from("RAII");
    }
    //堆上分配变量离开作用域时，调用drop函数释放资源
    //stack上的简单类型自动复制
    let x = 5;
    let y = x;
    //类似String的堆上分配内存的对象直接移动,这个部分参见doc
    let s1 = String::from("Hello");
    //此时s1把数据移动给s2,s1失效
    let s2 = s1;
    //println!("{}",s1);
    //不过也有clone的通用方法，便于进行非移动拷贝
    let s3 = s2.clone();
    println!("{} {}", s2, s3);
    //注意，实现了Copy trait的类型可以避开默认的移动语义直接复制
    //但是自身全部或者一部分使用了Drop的话，Copy trait就无法被实现
    //一般来讲，简单标量值都是直接复制的
    //函数传参也遵循此规则
    let s4 = s3.clone();
    take_it(s4); //s4直接被移动到形参里
    let num = 5;
    copy_it(num); //num直接被拷贝
    //函数也可以把所有权给回来
    let mut s4 = s3.clone();
    s4 = take_it_and_give_back(s4); //这里函数实参获取所有权后进行处理，然后把对应的值返回过来
    //s4不是mut的话会报错，给不可变变量赋值
    //可以利用多重返回值来把移动进函数的变量返回，同时获得函数的返回值
    let (s5, len) = get_length(s4);
    println!("{} {}", s5, len);
    //Rust中可以利用引用来解决这种笨重的传参问题
    //Rust中的,&为取引用运算符，*为解引用运算符,Rust中的引用更接近C/C++的指针
    //具体图详见doc中ownership.md
    //注意：函数作用域结束后引用会被丢弃，但是对应的值不会
    //因此获取引用作为函数参数也叫借用
    //注意：不可变借用不可以修改对应变量的值
    let len = get_len_by_ref(&s5);
    let mut s6 = String::new();
    println!("{} {}", s5, len);
    //Rust中允许创建可变引用
    let mutable_ref = &mut s6;
    let mut mutable_ref = 3;
    //注意传参时可变借用要明确写出
    change_value_by_mutable_ref(&mut s6);
    //为了防止数据竞争，不可以在同一作用域里创建指向同一变量的两个可变借用
    let r1 = &mut mutable_ref;
    //let r2 = &mut mutable_ref;
    //同样，同时创建指向同一变量的可变与不可变引用也不可以
    let mut x = 5;
    let mutref = &mut x;
    //let immuref = &x;
    //不过，由于一个变量的作用域从声明开始到最后一次使用，所以指向同一变量的不可变引用最后一次使用之后可以定义可变引用
    let mut y = 5;
    let immu1 = &x;
    let immu2 = &x;
    println!("{} {}", immu1, immu2);
    let mu1 = &mut y;
    println!("{}", mu1);
    //Rust编译器会自动识别悬垂引用，无法通过编译
    //字符串slice是String中一部份值的引用，类型为&str,中括号中的 [starting_index..ending_index] 指定的 range 创建一个 slice
    let s = String::from("Hello World");
    let hello = &s[0..5]; //取前五个字符
    let world = &s[6..11]; //取后五个字符
    //如果切片开头是字符串首或者结尾是字符串为那么指定的下标可以省略
    //切片的内存结构见对应的文档
    //注意：字符串切片索引须在有效UTF-8字符边界内
    //有了slice以后，很多问题也都能暴露出来
    let mut s = String::from("hello world");
    let word = first_word(&s);
    //s.clear()会尝试获取一个可变借用，但是word是一个不可变借用
    //s.clear(); // 错误!
    //注意：字符串字面值类型也是slice，&str是一个不可变引用
    //把&str作为参数可以更通用,类似C++中的string_view
    fn string_view(s: &str){
        println!("{}",s);
    }
    string_view("Hello World!"); //字符串字面量
    string_view(&s); //通过借用调用
    string_view(hello); //通过切片调用
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();//把字符串变成字节数组
    //利用模式匹配进行解构，获得对应字符的值及其对应下标
    //bytes.iter().enumerate()方法显示获得字节数组对应的迭代器，然后enumerate()函数对返回值进行包装得到一个字节数组
    for (idx, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..idx];
        }
    }
    &s[..]
}

fn take_it(some_string: String) {
    println!("{}", some_string);
}

fn copy_it(some_i32: i32) {
    println!("{}", some_i32);
}

fn take_it_and_give_back(a_string: String) -> String {
    println!("{}", a_string);
    a_string
}

fn get_length(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}

fn get_len_by_ref(s: &String) -> usize {
    s.len()
}

fn change_value_by_immutable_ref(s: &String) {
    //无法通过不可变借用修改变量值
    //s.push_str(" ");
}

fn change_value_by_mutable_ref(s: &mut String) {
    //声明为可变借用以后就没有这个问题了
    s.push_str("123");
}