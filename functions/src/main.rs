fn main() {
    another_function(42, 42);
    ///Rust是一个基于**表达式**的编程语言
    ///Rust中，语句指的是执行一些操作但是不返回值的指令
    ///表达式返回，计算并且产生一个值
    ///let关键字创建变量并绑定一个值是一个语句
    let statement = 6;
    ///语句不返回值，因此下面得表达式错误
    ///let err = (let statement = 6);
    ///Rust中，函数调用，新作用域代码块的运算结果，宏调用以及简单的计算都是表达式
    let expression = {
        let statement = statement + 1;
        statement * 2 //这个代码块是一个表达式，其结果为statement*2,然后赋值给expression变量
    };
    ///注意：表达式结尾加上分号以后就变成了语
    /// rust也有if得控制流，注意控制条件没有隐式类型转换，也不用加()括号
    let num = 6;
    if num % 4 == 0 {
        println!("divisible by 4!");
    } else if num % 3 == 0 {
        println!("divisible by 3!");
    } else if num % 2 == 0 {
        println!("divisible by 2!");
    } else {
        println!("can't divide by 4 3 2!")
    }
    /// 可以在let语句中使用if，因为if是一个表达式
    let num = if true {
        4
    } else {
        2
    };
    /// 不过if语句两边的返回值要一样
    ///Rust中有三种循环，loop,while和for
    let mut cnt = 0;
    loop {
        println!("Again!");
        cnt += 1;
        if cnt == 2 {
            break;
        }
    }
    /// loop是无限循环，除非里有break或者用户强制退出，否则会一直执行下去
    /// 如果break后面跟一个表达式，那么它直接返回对应的值
    let mut cnt = 0;
    let loop_result = loop {
        cnt += 1;
        if cnt == 10 {
            break cnt;
        }
    };
    println!("{}", cnt);
    /// while循环和C++的一致(除了强制不写括号)
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("LIFTOFF!!!");
    ///Rust的for和python的for差不多，都是遍历的可迭代集合
    let array = [10, 20, 30, 40, 50];
    for elem in array.iter()/*直接从可迭代集合中取元素来迭代*/{
        println!("The value is {}", elem);
    }
    for num in (1..4).rev()/*1到4做逆序输出，(1..4)产生了一个数字序列*/{
        println!("{}",num);
    }
}

///Rust的函数形参要给出类型注解
fn another_function(x: i32, y: i32) {
    println!("The value of y is {}", x);
    println!("The value of y is {}", y);
}

fn five() -> i32 /*Rust中需要用箭头声明返回值类型，否则默认为()*/ {
    5 //直接返回表达式5
}
