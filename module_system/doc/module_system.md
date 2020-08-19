# Rust中的模块系统

* Rust中一个包可以包含多个二进制crate项以及一个可选的crate库
* Rust中的模块系统包括：
  * 包：构建 测试 分享crate
  * crate：一个模块的树形结构，形成二进制或者库
  * 模块和use可以控制作用域与路径私有性
  * 路径：一个命名比如结构体、函数或者模块等项的方式

## 包和crate

* crate是一个二进制项或者库
* crate root是一个源文件，它构成了一个crate的根模块
* 包里包含一系列crate
* `cargo new`的时候，它遵循约定：对于二进制文件，`src/main.rs`是与包同名的二进制crate的根，库的话`src/lib.rs`是库crate的根
* crate根文件由cargo传给rustc构建库或者二进制
* `src/bin`中的源代码对应多个二进制crate，编译时被编译成独立的二进制crate
* 同crate一般分组一致

