use std::fs;
use std::io::prelude::*;
use std::io::stdin;

pub fn main() {
    println!("==========file-io===================");
    // 输出本文件路径
    let args = std::env::args();
    println!("{:?}", args);

    for arg in args {
        println!("{}", arg)
    }

    // 命令行输入
    println!("Please input: ");
    // let mut str_buf = String::new();
    // stdin()
    //     .read_line(&mut str_buf)
    //     .expect("Failed to read line.");

    // println!("You input line is \n {}", str_buf);

    // 新建文件写入
    fs::write(
        "/home/wangcheng/ext/rust-learning-project/src/read_file.txt",
        "测试文件写入谨慎使用，会覆盖全部数据。。",
    )
    .unwrap();
    let mut r_f =
        fs::File::create("/home/wangcheng/ext/rust-learning-project/src/read_file.txt").unwrap();
    r_f.write(b"Test file append \n").unwrap();
    r_f.write(b"Test file append2 \n").unwrap();
    // 打开文件追加写入, 也支持新建文件
    let mut a_f = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open("/home/wangcheng/ext/rust-learning-project/src/read_file.txt")
        .unwrap();
    a_f.write_all("ni hao wa \n".as_bytes()).unwrap();

    // 读取文件
    // string
    let text =
        fs::read_to_string("/home/wangcheng/ext/rust-learning-project/src/read_file.txt").unwrap();
    println!("{}", text);
    // 二进制
    let content = fs::read("/home/wangcheng/ext/rust-learning-project/src/read_file.txt").unwrap();
    println!("{:?}", content);

    // 文件流读取
    // 按照buffer的长度依次读取
    // 不需要手动close, rust会自动close文件
    let mut buffer = [0u8; 5];
    let mut f =
        fs::File::open("/home/wangcheng/ext/rust-learning-project/src/read_file.txt").unwrap();
    f.read(&mut buffer).unwrap();
    println!("{:?}", buffer);
    f.read(&mut buffer).unwrap();
    println!("{:?}", buffer);
}
