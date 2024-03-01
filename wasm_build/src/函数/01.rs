// fn add(i: i32, j: i32) -> i32 {
//     i + j
// }

// fn main() {
//     println!("{}", add(1, 2))
// }

//函数要点：蛇形命名法 fn add_two()=>{}

//函数参数

// fn main() {
//     another_function(5, 6.1)
// }

// fn another_function(x: i32, y: f32) {
//     println!("The value of x is:{}", x);
//     println!("The value of y is:{}", y);
// }

//函数返回
// fn plus_five(x: i32) -> i32 {
//     x + 5
// }

// fn main() {
//     let x = plus_five(5);
//     print!("The value of x is:{}", x);
// }

// fn plus_or_minus(x: i32) -> i32 {
//     if x > 5 {
//         return x - 5;
//     }
//     x + 5
// }

// fn main() {
//     let x = plus_or_minus(5);
//     println!("The value of x is:{}", x);
// }

//Rust 中的特殊返回类型

//无返回值
// use std::fmt::Debug;

// fn report<T: Debug>(item: T) {
//     println!("{:?}", item)
// }

// fn clear(text: &mut String) -> () {
//     *text = String::from("")
// }

// fn add(x: u32, y: u32) -> u32 {
//     x + y;
// }

//永不返回的发散函数

// fn dead_end() -> ! {
//     panic!("你已经到了穷途末路,崩溃吧！")
// }

// fn forever() -> ! {
//     loop {}
// }
