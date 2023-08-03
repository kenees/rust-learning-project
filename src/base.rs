// const A: i32 = 10;
// A = 100; error

// let mut a: i32 = 123; // 不安全 error
// a = 456;

pub fn base() {
    println!("----------base-----------");
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);
    let y = 2.0; // f64
    println!("The value of y is: {}", y);
    let y: f32 = 3.0; // f32
    println!("The value of y is: {}", y);
    // 运算符  // 不支持++，--
    let sum = 5 + 10;
    let difference = 95.5 - 1.0; // error : 95.5 - 1
    let product = 5 * 40;
    let quotient = 56.7 / 32.2; // 除
    let remainder = 43 % 5; // 求余

    println!("The value of + is: {}", sum);
    println!("The value of - is: {}", difference);
    println!("The value of * is: {}", product);
    println!("The value of / is: {}", quotient);
    println!("The value of % is: {}", remainder);

    // 元组
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x: {}, y: {}, z: {}", x, y, z);
    // 数组
    let arr = [1, 2, 3, 4, 5];
    println!("{}", arr[2]);
    let mut b = ["jan", "feb", "mar"];
    let c: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{}", c[2]);
    let d = [3; 5]; // [3,3,3,3,3]
    println!("The value of d: {},{},{},{}", d[0], d[1], d[4], d[3]);
    // arr[0] = 123; error, 数组arr不可变
    b[0] = "Jan";
    println!("The value of b: {}", b[0]);
}

// 1. const 常量名称建议大写, 且必须指定类型，不可变
// 2. let 不可变变量， 同一个变量名称可重复声明且类型不同（重影）
// 3. let mut 可变变量(只能改变值，不能改变类型)
// 4. 不建议全局变量，不安全

// 数据类型
// integer
// 位长度    有符号   无符号
// 8-bit	i8	    u8
// 16-bit	i16	    u16
// 32-bit	i32	    u32
// 64-bit	i64	    u64
// 128-bit	i128	u128
// arch	    isize   usize

// floating-point
// let x = 2.0 ;//f64
// let x:f32 = 3.0; // f32

// bool   true, false
// char
// 复合类型
// 1. 元组 let tup: (i32, f64, u8) = (500,6,4, 1);

// 进制
// 十进制： 98_222
// 十六进制： 0xff
// 八进制：  0o77
// 二进制：  0b1111_0000
// 字节（只能表示u8）   b'A'
