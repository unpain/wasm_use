fn main() {
    let a = [123, 456, 789, 123, 456];
    // let b = (123, 456, 789, 123, 456);
    let mut index = 0;
    for i in 0..=4 {
        if i < 4 {
            // println!("{}", a[index]);
            println!("{}", a[index])
        }
        index += 1;
    }
}
