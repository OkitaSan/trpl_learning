fn main() {
    println!("Hello, world!");
    let point1 = Point{x:1,y:2};
    let point2 = Point{x:1.0,y:2.0};
    //let err_point = Point{x:1,y:2.0}；类型不匹配,因为只声明了一个泛型参数,其余部分无法推断
    let point3 = PointTwoParameters{x:1,y:2.0}; //通过
}
//函数接受一个传入的i32的切片
fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
//函数泛型
//在函数参数签名前使用类型参数时必须在使用它之前就声明，然后就可以像普通类型一样使用
//不过函数体并不能适用所有可能的类型，因此不写上trait的话无法编译
fn largest_generic<T>/*声明泛型参数*/(list:&[T]) -> T
    where T:std::cmp::PartialOrd+Copy{
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
//结构体泛型
//先在结构体名后声明后使用
struct Point<T>{
    x:T,
    y:T
}
//方法定义中的泛型
//通用的泛型定义,要在impl和结构体/枚举名后都加T
impl<T> Point<T>{
    fn get_x(&self) -> &T{
        &self.x
    }
}
//也可以给某个类型特化方法,此时impl后不用再写内容,但是Point后面要写出具体类型
impl Point<f32>{
    fn distance_from_origin(&self) -> f32{
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
//改进后的Point
//这样{x:1,y:2.0}即为合法
struct PointTwoParameters<T,U>{
    x:T,
    y:U
}
//结构体方法签名中还可以额外再声明泛型类型
//T和U是结构体的,A和B是方法的
impl<T,U> PointTwoParameters<T,U>{
    fn mixup<A,B>(self,other:PointTwoParameters<A,B>) -> PointTwoParameters<T,B>{
        PointTwoParameters{
            x:self.x,
            y:other.y
        }
    }
    fn new(x:T, y:U) -> PointTwoParameters<T,U>{
        PointTwoParameters{x,y}
    }
}
//枚举中的泛型
enum EnumResult<T,E>{
    Success(T),
    Error(E)
}

