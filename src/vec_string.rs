/// 向量(vector)： 是一个存放多值得但数据结构， 该结构将相同类型的值线性的存放在内存中。
/// 向量是线性表， 在Rust中表示为： Vec<T>
/// 向量的使用方式类似列表List
///
use std::collections::HashMap;

pub fn main() {
    println!("==========vector===================");
    // push, 添加单个数据
    // append, 将一个向量拼接到另一个向量的尾部
    // get, 取出向量的值
    // vector[0], 下标取值
    // 遍历： for in
    let mut vector: Vec<i32> = Vec::new();
    let mut vector2: Vec<i32> = vec![1, 2, 3, 4, 5]; // 通过数组创建向量
    vector.push(1);
    vector.push(11);
    vector2.push(1);
    vector2.push(13);
    println!("{:?}", vector);
    println!("{:?}", vector2);

    vector.append(&mut vector2);
    println!("{:?}", vector);

    println!(
        "get vector [1]: {:?}",
        match vector.get(1) {
            Some(value) => value.to_string(),
            None => "None".to_string(),
        }
    );
    println!("get vector [0]: {:?}", vector[8]);

    // 遍历
    for v in &vector {
        println!("iter v: {}", v);
    }

    println!("==========String===================");
    // 新建字符串： String::new()
    // 字符串转换：to_string()
    // 字符串追加：
    //          push_str, // 追加&str
    //          push      // 追加字符
    //          +         // 拼接字符串
    //          format!   // 拼接字符串
    // 字符串长度：len()
    // 统计字符长度： str.chars().count(); // 每个中文，符号，数字，字符都占一个长度
    // 遍历字符串： for char in string.chars()
    // 根据索引获取字符： string.chars().nth(2);
    let mut string = String::new();
    let one = 1.to_string();
    let float = 1.3.to_string();
    let slice = "slice".to_string();

    string.push_str("aa");
    string.push('/');
    string = string + "b";
    string = string + &slice + "-" + &float;

    println!("string: {}", string);

    println!("format: string: {}", format!("{}-{}-{}", one, float, slice));

    let name = "王成！a1";
    let r_name = "wangcheng";
    println!(
        "name length: {}, name bytes length: {},  r_name length: {}",
        name.len(),
        name.chars().count(),
        r_name.len()
    );

    for ch in name.chars() {
        println!("{}", ch);
    }

    println!("name [2]: {:?}", name.chars().nth(2));

    println!("==========HashMap===================");
    let mut map = HashMap::new();

    map.insert("color", "red");
    map.insert("size", "10 m^2");

    // 更安全的插入数据
    map.entry("color").or_insert("blue");
    map.entry("age").or_insert("11");

    for p in map.iter() {
        println!("key: {:?}, value: {:?}", p.0, p.1);
    }
}
