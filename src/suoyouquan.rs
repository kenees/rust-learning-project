/// 所有权概念是为了让 Rust 在编译阶段更有效地分析内存资源的有用性以实现内存管理而诞生的概念。

/// Rust 中的每个值都有一个变量，称为其所有者。
/// 一次只能有一个所有者。
/// 当所有者不在程序运行范围时，该值将被删除。

pub fn test() {
    {
        // 在声明以前，变量 s 无效
        let s = "runoob";
        // 这里是变量 s 的可用范围
    }
    // 变量范围已经结束，变量 s 无效

    let s1 = String::from("hello");
    let s2 = s1; // 此时s1已经无效了， 一个值只能有一个所有者
    let s3 = String::from("world");
    let s4 = s3.clone(); // s3, s4都有效
    println!("s3={}, s4={}", s3, s4);

    // 涉及函数的所有权机制
    take();
    give(); // 返回所有权
    test4(); // 引用和租借
}

fn take() {
    let s = String::from("hello");
    // s 被声明有效

    takes_ownership(s);
    // s 的值被当作参数传入函数
    // 所以可以当作 s 已经被移动，从这里开始已经无效

    let x = 5;
    // x 被声明有效

    makes_copy(x);
    // x 的值被当作参数传入函数
    // 但 x 是基本类型，依然有效
    // 在这里依然可以使用 x 却不能使用 s
}

fn takes_ownership(some_string: String) {
    // 一个 String 参数 some_string 传入，有效
    println!("{}", some_string);
} // 函数结束, 参数 some_string 在这里释放

fn makes_copy(some_integer: i32) {
    // 一个 i32 参数 some_integer 传入，有效
    println!("{}", some_integer);
} // 函数结束, 参数 some_integer 是基本类型, 无需释放

fn give() {
    let s1 = gives_ownership();
    // gives_ownership 移动它的返回值到 s1

    let s2 = String::from("hello");
    // s2 被声明有效

    let s3 = takes_and_gives_back(s2);
    // s2 被当作参数移动, s3 获得返回值所有权
} // s3 无效被释放, s2 被移动, s1 无效被释放.

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    // some_string 被声明有效

    return some_string;
    // some_string 被当作返回值移动出函数
}

fn takes_and_gives_back(a_string: String) -> String {
    // a_string 被声明有效

    a_string // a_string 被当作返回值移出函数
}

fn test4() {
    let s1 = String::from("abcd");
    let s2 = &s1; // 复制了栈中的内存地址给s2, s2 只能使用， 不能修改
                  // s2 = "defg"; // error

    println!("s1 is {}, s2 is {}", s1, s2);

    let mut s3 = String::from("abcddd");
    let s4 = &mut s3; //
                      // let s5 = &mut s3; // error, 不能多次引用， 与没有mut修饰的区别
    s4.push_str(" defg");
    println!(" s4 is {}", s4);
}
