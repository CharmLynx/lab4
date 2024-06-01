fn take_by_reference(x: &mut i32) {
    *x += 1;
}

fn take_by_value(y: i32) -> i32 {
    y += 1;
    y
}
fn main() {
    let mut x = 42;
    take_by_reference(&mut x);
    let y = 42;
    let y = take_by_value(y);
    println!("x: {}, y: {}", x, y);
}
