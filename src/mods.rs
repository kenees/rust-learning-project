// rust组织管理
// 箱: Crate 二进制程序文件或者库文件，存在于包中
// 包：package  一个包最多包含一个库"箱"，可以包含任意数量的二进制"箱"，但是至少包含一个"箱"（不管是库还是二进制"箱"）
// 模块： Module 类似Java 组织功能模块的主要单位是类，而 JavaScript 组织模块的主要方式是 function。

pub mod nation {
    pub mod government {
        pub fn govern() {
            println!("govern...");
        }
    }

    pub mod congress {
        pub fn legislate() {
            println!("legislate...");
        }

        pub fn govern() {
            println!("as govern...");
        }
    }

    pub use congress::legislate;
}

pub fn message() -> String {
    println!("-----------mods----------");
    return String::from("This is the 2nd module.");
}
