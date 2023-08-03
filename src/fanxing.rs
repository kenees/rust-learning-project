pub fn max(array: &[i32]) -> i32 {
    let mut max_index = 0;
    let mut i = 0;
    while i < array.len() {
        if array[i] > array[max_index] {
            max_index = i;
        }
        i += 1;
    }
    array[max_index]
}

// 泛型
// 不是所有类型都可比较， 该函数只是描述泛型的语法格式
// pub fn max2<T>(array: &[T]) -> T {
//     let mut max_index = 0;
//     let mut i = 0;
//     while i < array.len() {
//         if array[i] > array[max_index] {
//             max_index = i;
//         }
//         i += 1;
//     }

//     array[max_index]
// }

// Demo
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}

impl Point<f64> {
    fn x1(&self) -> f64 {
        (&self.x + &self.y)
    }
}

#[derive(Debug)]
struct Point2<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point2<T, U> {
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

// 泛型特性, 可以实现多个
struct Person {
    name: String,
    age: u8,
}

trait Descriptive {
    fn describe(&self) -> String;
}

impl Descriptive for Person {
    fn describe(&self) -> String {
        format!("{}, {}", self.age, self.name)
    }
}

pub fn main() {
    let a = [1, 2, 3, 5, 7, 9, 4];
    println!("max is : {}", max(&a));
    // println!("max2 is : {}", max2(&a)); // error
    let p = Point { x: 12, y: 11 };
    println!("p.x = {}, p.x = {}", p.x(), p.x);
    println!("p.y = {}, p.y = {}", p.y(), p.y);
    let p1 = Point { x: 12.1, y: 11.1 };
    println!("x1 = {}", p1.x1());
    let p2 = Point2 { x: 11.2, y: 11.3 };
    let p22 = Point2 { x: 1, y: 2 };
    let p33 = p22.mixup(p2);
    println!("mixup: {}, {}, {:?}", p33.x, p33.y, p33);
    // 泛型特性
    let cali = Person {
        name: String::from("Cali"),
        age: 23,
    };

    println!("{}", cali.describe());
}
