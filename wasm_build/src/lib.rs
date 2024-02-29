use wasm_bindgen::prelude::*;

//计算两个数相加
#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
//计算两个数相减
#[wasm_bindgen]
pub fn sub(a: i32, b: i32) -> i32 {
    a - b
}
//计算两个数相乘
#[wasm_bindgen]
pub fn multi(a: f32, b: f32) -> f64 {
    (a * b).into()
}
//计算两个数相除
#[wasm_bindgen]
pub fn divide(a: f32, b: f32) -> f64 {
    (a / b).into()
}

#[wasm_bindgen]
pub fn main() {
    // loop {
    //     println!("{}", add(1, 2))
    // }
    // let mut counter = 0;
    // let result = loop {
    //     counter += 1;
    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };
    // println!("The result is {}", result)
    divide(4.0, 3.0);
}
