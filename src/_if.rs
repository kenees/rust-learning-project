/// if 表达式的值必须是 bool
/// 可以不写小括号
/// 不支持单条语句不加{}
///
pub fn test() {
    println!("-----------if----------");
    let a = 10;
    if a > 0 {
        println!("a > 0");
    } else {
        println!("a <= 0");
    }

    let num = if a > 0 { 1 } else { -1 }; // 必须有两个{}，且类型一致
    println!("{}", num);
}
