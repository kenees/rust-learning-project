// 理解的差点意思， 还得再研究研究
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub fn main() {
    let str1 = String::from("abcd");
    let str2 = "zxv";

    let result = longest(str1.as_str(), str2);
    println!("The longest string is {}", result);

    //
    let string1 = String::from("wang");
    let result1;
    {
        let string2 = "cheng ge";
        result1 = longest(string1.as_str(), string2);
    }
    println!("The longest string is {}", result1);
}
