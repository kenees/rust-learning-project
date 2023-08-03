use std::fs::File;
/**
    Rust 有一套独特的处理异常情况的机制，它并不像其它语言中的 try 机制那样简单。
    首先，程序中一般会出现两种错误：可恢复错误和不可恢复错误。
        1. 可恢复错误的典型案例是文件访问错误，如果访问一个文件失败，有可能是因为它正在被占用，是正常的，我们可以通过等待来解决。
        2. 但还有一种错误是由编程中无法解决的逻辑错误导致的，例如访问数组末尾以外的位置。

    // RUST_BACKTRACE=1 cargo run
    // 设置环境变量用来回溯错误信息
*/
use std::io;
use std::io::Read;

pub fn main() {
    // panic!("error occured"); // 不可恢复错误使用panic!宏, 一定会导致程序受到致命的打击而终止运行。
    println!("Hello, Rust");

    let f = File::open("hello.txt"); // 可回复错误
                                     // Test1
                                     // match f {
                                     //     Ok(file) => {
                                     //         println!("File open successfully");
                                     //     }
                                     //     Err(err) => {
                                     //         println!("Failed to open the file.");
                                     //     }
                                     // }
                                     //  Test 2
    if let Ok(file) = f {
        println!("File open successfully");
    } else {
        println!("Failed to open the file.");
    }

    // 可回复错误按照不可回复处理
    // let f1 = File::open("hello1.txt").unwrap(); // 不可传递指定信息
    // let f2 = File::open("hello2.txt").expect("Filed to open"); // 不可传递指定信息

    // 自定义错误传递
    main2();
    // 将异常传递出去解决
    main3();
}

fn f(i: i32) -> Result<i32, bool> {
    if i >= 0 {
        Ok(i)
    } else {
        Err(false)
    }
}

// fn g(i: i32) -> Result<i32, bool> {
//     let t = f(i);
//     return match t {
//         Ok(i) => Ok(i),
//         Err(b) => Err(b),
//     };
// }

// 简化g的代码
fn g1(i: i32) -> Result<i32, bool> {
    let t = f(i)?; // 跟随？直接将Err传递出去
    Ok(t) // 因为确定 t 不是 Err, t 在这里已经是 i32 类型
}

fn main2() {
    // let r = f(10000);
    // let r = g(10000);
    let r = g1(10000);

    if let Ok(v) = r {
        println!("Ok: f(-1) = {}", v);
    } else {
        println!("Err");
    }
}

// Kind， 获取err类型

// 读取文件并返回Result
fn read_text_from_file(path: &str) -> Result<String, io::Error> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn main3() {
    let str_file = read_text_from_file("hello.txt");
    match str_file {
        Ok(s) => println!("{}", s),
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => {
                println!("no such file");
            }
            _ => {
                println!("cannot read the file..");
            }
        },
    }
}
