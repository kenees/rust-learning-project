mod _enum;
mod _if;
mod _string;
mod _struct;
mod _while_for_loop;
mod base;
mod class;
mod error;
mod fanxing;
mod fanxing_trait;
mod file_io;
mod fns;
mod life_cycle;
mod mods;
mod slice;
mod spawn;
mod suoyouquan;
mod use_std;
mod vec_string;

use std::thread::spawn;

use mods::nation::congress::govern; //use 关键字能够将模块标识符引入当前作用域：
use mods::nation::government::govern as as_govern; // as 可以为标识符添加别名

fn main() {
    base::base(); // 基础数据类型

    fns::my_fn(); // 函数

    println!("{}", mods::message());
    govern(); // use的使用可mod模块化
    mods::nation::congress::legislate(); // use的使用可mod模块化
    as_govern(); // use的使用可mod模块化
    mods::nation::legislate(); // mods文件中已经提前使用use了， 这里可以直接使用

    use_std::print_pi(); // 使用系统库

    error::main(); // 异常处理

    _if::test();

    _while_for_loop::test();

    suoyouquan::test();

    slice::test();

    _string::test(); // String, String -> str

    _struct::test(); // struct, impl, trait

    _enum::test(); // enum, match

    // 泛型
    fanxing::main();
    fanxing_trait::main();

    // 生命周期
    life_cycle::main();

    // 文件， io
    file_io::main();

    // rust 集合与字符串
    vec_string::main();

    // 面向对象
    let obj = class::ClassName::new(1233);
    obj.public_method();

    // 多线程, 闭包
    spawn::main();
}
