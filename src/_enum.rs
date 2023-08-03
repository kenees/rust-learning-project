#[derive(Debug)]

enum Book {
    Papery(u32),        // 元组类型
    Electronic(String), // 元组类型

    // 可为属性命名
    Papery2 { index: u32 },      // 结构体
    Electronic2 { url: String }, // 结构体
}

// Option枚举类
enum Option<T> {
    Some(T),
    None,
}

pub fn test() {
    println!("--------enum----------");
    let book = Book::Papery(123);
    let book2 = Book::Electronic(String::from("https://www.baidu.com"));
    let book3 = Book::Papery2 { index: 199 };
    let book4 = Book::Electronic2 {
        url: String::from("https://www.baidu.com"),
    };

    println!("{:?}", book);
    println!("{:?}", book2);
    println!("{:?}", book3);
    println!("{:?}", book4);

    match book3 {
        Book::Papery2 { index } => {
            let i = index + 1;
            println!("Papery2 book: {}", i);
        }
        Book::Electronic2 { url } => {
            println!("Electronic2 book: {}", url);
        }
        Book::Papery(id) => {
            println!("Papery: {}", id);
        }
        Book::Electronic(str) => {
            println!("Papery: {}", str);
        }
    }

    let opt: Option<&str> = Option::None;
    match opt {
        Option::Some(some) => {
            println!("some: {}", some);
        }
        Option::None => {
            println!("opt is none")
        }
    }
}

//    match 块也可以当作函数表达式来对待，它也是可以有返回值的，返回值类型必须一致
//    match 枚举类实例 {
//        分类1 => 返回值表达式，
//        分类2 => 返回值表达式，
//   }
//
//   枚举类中有元组， 在match中需要临时指定一个名字
//  Book::Papery(id) => {
//     println!("Papery: {}", id);
//  }
//  Book::Electronic(str) => {
//     println!("Papery: {}", str);
//  }
//
