// 函数, 命名规范 小写字母下划线
// fn c_d (a) {}

pub fn my_fn() {
    println!("----------fn-----------");
    println!("begin...");
    send();
    println!("1 + 2 = {}", add(1, 2));
    test2();
    test3();
}

fn send() {
    println!("abc");
}

// Rust 中定义函数如果需要具备参数必须声明参数名称和类型：
fn add(a: i32, b: i32) -> i32 {
    // return (a + b).into();
    return a + b;
}

fn test2() {
    let x = 5;

    // 函数体表达式， 最后一个步骤是表达式，此表达式的结果值是整个表达式块所代表的值
    // 注意：x + 10 之后没有分号，否则它将变成一条语句！
    let y = {
        let x = 20;
        x + 10
    };

    println!("x: {}, y: {}", x, y);
}

// 函数嵌套
//  Rust 不支持自动返回值类型判断, 如果没有明确声明函数返回值的类型，函数将被认为是"纯过程"，不允许产生返回值，
fn test3() {
    fn five() -> i32 {
        5
    }
    println!("five(): {}", five());

    // return 11; error: 因为没有定义返回值类型， 不允许返回
}
