//Rust中集成测试测试的是外部公有API
//需要单独放一个src/tests目录,放测试文件,每个文件被当作单独的crate被编译
use test_module; //每个tests目录中的项目都是独立crate,因此需要在每个文件中导入库
//不需要写#[cfg(test)],因为只会在cargo test的时候执行tests文件夹里的测试
#[test]
fn it_adds_two(){
    assert_eq!(4,test_module::)
}
