pub fn test() {
    // 1. String 类型字符串
    let s = String::from("abcdef");

    // &str
    let ss = "abcdef"; // 这是一个引用, &str类型

    // String -> &str
    let aa = String::from("jhigklmn"); // String
    let bb = &aa[1..3]; // &str
    let cc = &aa[..]; // 快速将String转成 &str

    println!("{}, {}, {}, {}, {}", s, ss, aa, bb, cc);

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
    // println!("name [2]: {:?}", name.chars()[2]);
}
