pub fn test() {
    println!("-----------_while_for_loop----------");
    // while
    let mut num = 1;
    while num < 10 {
        num += 1;
        println!("{}", num);
    }
    println!("{}", num);

    // for
    let arr = [10, 11, 12, 13, 15];
    for i in arr.iter() {
        println!("值：{}", i);
    }
    for i in 1..5 {
        println!("a[{}]: {}", i, arr[i]);
    }

    // loop, 无限循环
    let s = ['R', 'U', 'N', 'O', 'O', 'B', 'C'];
    let mut i = 0;
    loop {
        let ch = s[i];
        if ch == 'O' {
            break;
        }
        println!("'{}'", ch);
        i += 1;
    }
    // loop break(v), 返回一个值
    let location = loop {
        let ch = s[i];
        if ch == 'O' {
            break i;
        }
        i += 1;
    };

    println!("loop break O index: {}", location);
}
