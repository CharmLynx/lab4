fn take_by_reference(x: &mut i32) {
    *x += 1;
}

fn take_by_value(mut x: i32) -> i32 {
    x += 1;
    x
}
fn main() {
    let mut x = 42;
    take_by_reference(&mut x);
    let y = take_by_value(x);
    println!("x: {}, y: {}", x, y);
}
