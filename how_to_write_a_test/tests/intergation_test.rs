//Rust中集成测试测试的是外部公有API
//需要单独放一个src/tests目录,放测试文件,每个文件被当作单独的crate被编译
use how_to_write_a_test as tmodule; //每个tests目录中的项目都是独立crate,因此需要在每个文件中导入库
//不需要写#[cfg(test)],因为只会在cargo test的时候执行tests文件夹里的测试
//集成测试也可以通过特定规则来实现对应的测试,和单元测试一样
//集成测试可能会用到相同的行为,为了避免集成测试中的文件被测试
//需要在test目录下新建文件夹然后在那里实现测试的公共模块
//注意模块分割在不同文件中的结构
mod common;
#[test]
fn it_adds_two(){
    common::setup();
    assert_eq!(4,tmodule::add_two(2));
}
