fn main() {
    let mut a: i32 = 5;
    modify_value(a);
    println!("Value after modification: {}", a);
}

fn modify_value(mut x: i32) {
    x = 10;
}