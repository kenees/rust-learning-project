use std::thread;
use std::time::Duration;

/// 主线程结束了， 子线程也会终止
///
///

fn spawn_function() {
    for i in 0..5 {
        println!("spawned thread function input: {}", i);
        thread::sleep(Duration::from_millis(1));
    }
}

// 闭包
fn closures() {
    // |参数1, 参数2, ....| -> 返回值类型 {
    // main...
    // }

    let inc = |a: i32, b: i32| -> i32 { a + b };

    println!("a + b = inc(2, 4) = {}", inc(2, 4));
}

pub fn main() {
    println!("========== thread ===========");
    thread::spawn(spawn_function);
    // 推荐闭包传参数
    thread::spawn(|| {
        for i in 0..5 {
            println!("spawned thread function input: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // join, 可以让子线程结束后再停止程序
    // 类似 async await的效果
    let handle = thread::spawn(|| {
        for i in 0..5 {
            println!("join spawned thread print {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();
    // let _ = handle.thread();
    closures();

    for i in 0..3 {
        println!("main thread function input: {}", i);
        thread::sleep(Duration::from_millis(i));
    }

    // move 强制所有权迁移
    // 子线程访问当前函数的资源
    let s = "hello";
    let handle2 = thread::spawn(move || {
        println!("{}", s);
    });
    handle2.join().unwrap();

    // 消息传递
    // 通过通道 channel , 包括 transmitter: 发送者， receiver: 接受者
    let (tx, rx) = std::sync::mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
