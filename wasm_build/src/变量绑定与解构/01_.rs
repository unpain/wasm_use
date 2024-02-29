// fn greet_world() {
//     let southern_germany = "Grüß Gott!";
//     let chinese = "世界 你好";
//     let english = "world hello";
//     let regions = [southern_germany, chinese, english];
//     for region in regions.iter() {
//         print!("{}", &region)
//     }
// }
// fn main() {
//     greet_world();
// }

// fn main() {
//     let penguin_data = "\
//     common name,length(cm)
//     Little penguin,33
//     Yellow-eyed penguin,65
//     Fiordland penguin,60
//     Invalid,data
//     ";
//     let records = penguin_data.lines();

//     for (i, record) in records.enumerate() {
//         if i == 0 || record.trim().len() == 0 {
//             continue;
//         }
//         //声明一个fields变量，类型是Vec
//         //Vec是vector的缩写，是一个可伸缩的集合类型，可以认为是一个动态数组
//         //<_>表示Vec中的元素类型由编译器自行推断，在很多场景下，都会帮我们胜却不少功夫
//         let fields: Vec<_> = record.split(',').map(|field| field.trim()).collect();
//         if cfg!(debug_assertions) {
//             eprintln!("debug: {:?} -> {:?}", record, fields);
//         }
//         let name = fields[0];
//         // 1. 尝试把 fields[1] 的值转换为 f32 类型的浮点数，如果成功，则把 f32 值赋给 length 变量
//         //
//         // 2. if let 是一个匹配表达式，用来从=右边的结果中，匹配出 length 的值：
//         //   1）当=右边的表达式执行成功，则会返回一个 Ok(f32) 的类型，若失败，则会返回一个 Err(e) 类型，if let 的作用就是仅匹配 Ok 也就是成功的情况，如果是错误，就直接忽略
//         //   2）同时 if let 还会做一次解构匹配，通过 Ok(length) 去匹配右边的 Ok(f32)，最终把相应的 f32 值赋给 length
//         //
//         // 3. 当然你也可以忽略成功的情况，用 if let Err(e) = fields[1].parse::<f32>() {...}匹配出错误，然后打印出来，但是没啥卵用
//         if let Ok(length) = fields[1].parse::<f32>() {
//             //输出到标准输出
//             println!("{},{}cm", name, length);
//         }
//     }
// }

// fn main(){
//     let mut x = 5;
//     println!("The value of x is :{}",x);
//     x = 6;
//     println!("The value of x is :{}",x);
// }

// fn main(){
//     let _x = 5;
//     let _y = 10;
// }

//变量解构
// fn main() {
//     let (a, mut b): (bool, bool) = (true, false);
//     println!("a = {:?},b = {:?}", a, b);
//     b = true;
//     assert_eq!(a, b)
// }

//解构式赋值
// struct Struct {
//     e: i32,
// }
// fn main() {
//     let (a, b, c, d, e);
//     (a, b) = (1, 2);
//     [c, .., d, _] = [1, 2, 3, 4, 5];
//     Struct { e, .. } = Struct { e: 5 };
//     // assert_eq!([1, 2, 3, 4, 5], [a, b, c, d, e]);
//     println!("a = {:?},b = {:?},c= {:?},d={:?},e={:?}", a, b, c, d, e)
// }

// const MAX_POINTS: u32 = 100_000;
// //变量遮蔽
// fn main() {
//     let x = 5;
//     {
//         //在main函数的作用域内对之前的x进行遮蔽
//         let x = MAX_POINTS;
//         println!("The value of x in the inner scope is:{}", x);
//     }
//     println!("The value of x is:{}", x);
// }
// fn main() {
//     //字符串类型
//     let spaces = "   ";
//     //usize数值类型 不用let编译器会报错
//     let spaces = spaces.len();
//     println!("spaces is: {}", spaces)
// }

//整型溢出
// fn main() {
//     let a: u8 = 255;
//     let b = a.wrapping_add(20);
//     println!("{}", b);//19 溢出部分256-0 257-1，依此类推得出275-19（补码）
// }

// 浮点类型
// fn main() {
//     let x = 2.0; // f64
//     let y: f32 = 3.0; // f32
//     println!("x = {:?},y={:?}", x, y)
// }

//避免在浮点数上测试相等性
//当结果在数学上可能存在未定义时，需要格外地小心

// fn main() {
//     let a: f64 = 0.1;
//     let b: f64 = 0.2;
//     let c = a + b;
//     let d:f64 = 0.3;
//     assert!(c == d);
// }

// fn main() {
//     let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
//     let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);
//     println!("abc (f32)");
//     println!("0.1+0.2:{:x}", (abc.0 + abc.1).to_bits());
//     println!("0.3:{:x}", (abc.2).to_bits());
//     println!();

//     println!("xyz(f64)");
//     println!("0.1+0.2:{:x}", (xyz.0 + xyz.1).to_bits());
//     println!("0.3:{:x}", (xyz.2).to_bits());
//     println!();

//     assert!(abc.0 + abc.1 == abc.2);
//     assert!(xyz.0 + xyz.1 == xyz.2);
// }

//NaN不能与任何一个数字比较
// fn main(){
//     let x = (-42.0_f32).sqrt();
//     assert_eq!(x,x);
// }
//使用 is_nan来判断该数字是不是NaN
// fn main(){
//     let x = (-42.0_f32).sqrt();
//     if x.is_nan(){
//         println!("未定义的数学行为")
//     }
// }

//数字运算
// fn main() {
//     //加法
//     let sum = 5 + 10;
//     //减法
//     let diff = 95.5 - 4.3;
//     //乘法
//     let product = 4 * 30;
//     //除法
//     let quotient = 56.7 / 32.2;
//     //求余
//     let remainder = 43 % 5;

//     println!(
//         "sum:{:?},diff:{:?},product:{:?},quotient:{:?},remainder:{:?}",
//         sum, diff, product, quotient, remainder
//     );
// }

//数字运算综合示例
// fn main() {
//     //编译器会进行自动推导，给予twenty i32类型
//     let twenty = 20;
//     //类型标注
//     let twenty_one: i32 = 21;
//     //通过类型后缀的方式进行类型标注：22时i32类型
//     let twenty_two = 22i32;

//     //只有同样类型，才能运算
//     let addition = twenty + twenty_one + twenty_two;
//     println!("{}+{}+{} = {}", twenty, twenty_one, twenty_two, addition);

//     //对于较长的数字，可以用_进行分割，提升可读性
//     let one_million: i64 = 1_000_000;
//     println!("{}", one_million.pow(2));

//     //定义一个f32数组，其中42.0会自动被推导为f32类型
//     let forty_twos = [42.0, 42f32, 42.0_f32];

//     //打印数组中的第一个值，并控制小数位为2位
//     println!("{:.2}", forty_twos[0]);
// }

// fn main() {
//     //二进制为000000010
//     let a: i32 = 2;
//     //二进制为00000011
//     let b: i32 = 3;
//     println!("(a & b) value is {}", a & b);
//     println!("(a | b) value is {}", a | b);
//     println!("(a ^ b) value is {}", a ^ b);
//     println!("(!b) value is {}", !b);
//     println!("(a << b) value is {}", a << b);
//     println!("(a >> b) value is {}", a >> b);
//     let mut a = a;
//     // 注意这些运算符除了!之外都可以加上=进行赋值（因为!=要用来判断不等于）
//     a <<= b;
//     println!("(a << b) value is {}", a);
// }

//序列range
// fn main() {
//     for i in 1..=5 {
//         println!("{}", i);
//     }
// }
// fn main() {
//     for i in 'a'..='z' {
//         println!("{}", i);
//     }
// }

//有理数和复数
// use num::complex::Complex;

// fn main() {
//     let a = Complex { re: 2.1, im: -1.2 };
//     let b = Complex::new(11.1, 22.2);
//     let result = a + b;
//     println!("{}+{}i", result.re, result.im);
// }

//字符、布尔、单元类型
// fn main() {
//     let c = 'z';
//     let z = 'ℤ';
//     let g = '国';
//     let heart_eyed_cat = '😻';
//     println!(
//         "{},{},{},{}",
//         c,
//         z,
//         g,
//         std::mem::size_of_val(&heart_eyed_cat)
//     );
// }

// fn main() {
//     let _t = false;
//     let f: bool = true;
//     if f {
//         println!("这是一段毫无意义的代码")
//     }
// }

// 移除某个部分让代码工作
// fn main() {
//     let x: i32 = 5;
//     let mut y = 5;

//     y = x;

//     let z = 10; // 这里 z 的类型是? i32
// }

// 填空
// fn main() {
//     let v: u16 = 38_u8 as u16;
// }

// // 修改 `assert_eq!` 让代码工作
// fn main() {
//     let x = 5;
//     assert_eq!("i32".to_string(), type_of(&x));
// }

// // 以下函数可以获取传入参数的类型，并返回类型的字符串形式，例如  "i8", "u8", "i32", "u32"
// fn type_of<T>(_: &T) -> String {
//     format!("{}", std::any::type_name::<T>())
// }

// 填空，让代码工作
// fn main() {
//     assert_eq!(i8::MAX, 127);
//     assert_eq!(u8::MAX, 255);
// }

// // 解决代码中的错误和 `panic`
// fn main() {
//     let v1 = 247_u8 + 8;
//     let v2 = u8::checked_add(247, 8).unwrap();
//     println!("{},{}",v1,v2);
//  }

// // 修改 `assert!` 让代码工作
// fn main() {
//     let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
//     assert!(v == 1597);
// }

// // 将 ? 替换成你的答案
// fn main() {
//     let x = 1_000.000_1;
//     // let y: f32 = 0.12; // f32
//     // let z = 0.01_f64; // f64
//     assert_eq!(type_of(&x), "f64".to_string());
//     println!("success");
// }

// fn type_of<T>(_: &T) -> String {
//     format!("{}", std::any::type_name::<T>())
// }

// fn main() {
//     assert!(0.1f32+0.2f32==0.3);
// }

// fn main() {
//     assert!((0.1_f64 + 0.2 - 0.3).abs() < 0.001);
// }

// fn main() {
//     let mut sum = 0;
//     for i in -3..2 {
//         sum += i
//     }

//     assert!(sum == -5);

//     for c in 'a'..='z' {
//         println!("{}", c as u8);
//     }
// }



// 填空，并解决错误
// fn main() {
//     // 整数加法
//     assert!(1u32 + 2 == 3);

//     // 整数减法
//     assert!(1i32 - 2 == -1);
//     assert!(1i8 - 2 == -1);
    
//     assert!(3 * 50 == 150);

//     assert!(9.6f32 / 3.2f32 == 3.0); // error ! 修改它让代码工作

//     assert!(24 % 5 == 4);
    
//     // 逻辑与或非操作
//     assert!(true && false == false);
//     assert!(true || false == true);
//     assert!(!true == false);

//     // 位操作
//     println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
//     println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
//     println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
//     println!("1 << 5 is {}", 1u32 << 5);
//     println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
// }


// 修改2处 `assert_eq!` 让代码工作

// use std::mem::size_of_val;
// fn main() {
//     let c1 = 'a';
//     assert_eq!(size_of_val(&c1),4); 

//     let c2 = '中';
//     assert_eq!(size_of_val(&c2),4); 

//     println!("Success!")
// } 


// // 修改一行让代码正常打印
// fn main() {
//     let c1 = '中';
//     print_char(c1);
// }

// fn print_char(c : char) {
//     println!("{}", c);
// }


// 使成功打印
// fn main() {
//     let _f: bool = false;

//     let mut t = true;
//     t = !t;
//     if !t {
//         println!("Success!")
//     }
// } 

// fn main() {
//     let f = true;
//     let t = true || false;
//     assert_eq!(t, f);

//     println!("Success!")
// }



// 让代码工作，但不要修改 `implicitly_ret_unit` !
// fn main() {
//     let _v: () = ();

//     let _v1 = (2, 3);
//     assert_eq!(_v, implicitly_ret_unit());

//     println!("Success!")
// }

// fn implicitly_ret_unit() {
//     println!("I will return a ()")
// }


// 让代码工作：修改 `assert!` 中的 `4` 
// use std::mem::size_of_val;
// fn main() {
//     let unit: () = ();
//     assert!(size_of_val(&unit) == 0);

//     println!("Success!")
// }
