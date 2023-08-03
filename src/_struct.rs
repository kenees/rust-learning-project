#[derive(Debug)] // 导入调试库

struct Site {
    sex: String,
    name: String,
    age: u32,
}

// 元组结构体
struct Color(u8, u8, u8);
struct Point(f64, f64);

// 单元结构体， 一种象征，无需任何成员
// struct UnitStruct;

pub fn test() {
    let name = String::from("wang");

    let my_site = Site {
        sex: String::from("man"),
        name, // 等同于 name: name
        age: 11,
    };

    let you_sit = Site {
        sex: String::from("woman"),
        ..my_site
    };

    let color1 = Color(1, 2, 3);
    let point = Point(0.0, 0.0);

    println!("color1 = ({}, {}, {})", color1.0, color1.1, color1.2);
    println!("point = ({}, {})", point.0, point.1);

    println!("you_sit is {:?}", you_sit); // 通过 #[derive(Debug)] 和 {:?} 输出结构体
    println!("you_sit is {:#?}", you_sit); // 通过 #[derive(Debug)] 和 {:#?} 输出结构体带输出格式

    // 结构体方法
    let rect1 = Rectangle {
        width: 12,
        height: 30,
    };
    let rect2 = Rectangle {
        width: 32,
        height: 11,
    };

    let rect3 = Rectangle::create(5, 10);

    println!("rect1's area is: {}", rect1.area());
    println!("rect2's area is: {}", rect1.wider(&rect2));
    println!("rect3's area is: {}", rect3.area());
    println!("TODO trait: ");
}

struct Rectangle {
    width: u32,
    height: u32,
}

// impl 关键字： 定义结构体成员方法
impl Rectangle {
    // 结构体方法
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 结构体方法
    fn wider(&self, rect: &Rectangle) -> bool {
        self.width > rect.width
    }

    // 结构体函数(没有self)， 静态方法
    fn create(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
}

// TODO
// trait 接口
trait Area {
    fn area(&self) -> f64;
}
