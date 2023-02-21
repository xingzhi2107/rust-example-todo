mod model;

fn main() {
    let s = "1. [ ] 你好, world! 😊~";
    let mut count = 0;
    for char in s.chars() {
        println!("{}", char);
        count = count + 1
    }
    println!("count: {}", count);
    println!("len: {}", s.len());
}
