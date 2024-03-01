// fn main() {
//     let a = [123, 456, 789, 123, 456];
//     // let b = (123, 456, 789, 123, 456);
//     let mut index = 0;
//     for i in 0..=4 {
//         if i < 4 {
//             // println!("{}", a[index]);
//             println!("{}", a[index])
//         }
//         index += 1;
//     }
// }

// fn add_with_extra(x: i32, y: i32) -> i32 {
//     let x = x + 1;
//     let y = y + 5;
//     x + y
// }

// fn main() {
//     let x = 1i32;
//     let y = 2i32;
//     println!("{}+{}+6={:?}", x, y, add_with_extra(x, y))
// }

//语句

// fn main() {
//     let a = 8;
//     let b: Vec<f64> = Vec::new();
//     let (a, c) = ("hi", false);
// }

// fn main(){
//     let b = (let a=8);//错误写法
// }

//表达式
// fn main() {
//     let y = {
//         let x = 3;
//         x + 1 //表达式不能包含分号
//     };
//     println!("The value of y is:{}", y)
// }

//表达式如果不返回任何值，会隐式地返回一个 () 。
// fn ret_unit_type() {
//     let x = 1;
//     //if 语句块也是一个表达式,因此可以用于赋值，也可以直接返回
//     let _y = if x % 2 == 1 { "odd" } else { "even" };
// }
// fn main() {
//     assert_eq!(ret_unit_type(), ())
// }
