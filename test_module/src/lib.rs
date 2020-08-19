#[cfg(test)]//表示只在cargo test命令时执行测试,单元测试中要用到
//单元测试独立的测试每个单元
//要与它测试的代码共同存放在位于src目录下的相同文件中
//规范是在每个文件中创建包含测试函数的 tests 模块，并使用 cfg(tests) 标注模块。
//Rust可以测试私有模块
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
