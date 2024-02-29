use wasm_bindgen::prelude::*;

//计算两个数相加
#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn main() {
    // loop {
    //     println!("{}", add(1, 2))
    // }
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result)
}
