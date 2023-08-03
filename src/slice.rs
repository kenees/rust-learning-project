///
/// ..y 等价于 0..y
/// x.. 等价于位置 x 到数据结束
/// .. 等价于位置 0 到结束
///
///

pub fn test() {
    let s = String::from("broadcast");

    let part1 = &s[0..4]; // part1 不可修改
    let part2 = &s[4..9]; // part2 不可修改

    println!("{}={}+{}", s, part1, part2);

    let arr = [11, 22, 33, 44, 55];
    let part = &arr[1..3];
    for i in part.iter() {
        println!("{}", i)
    }
}
