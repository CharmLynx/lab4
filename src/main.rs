fn take_by_reference(x: &mut i32) {
    *x += 1;
}

fn take_by_value(y: i32) -> i32 { //не спрацює, оскільки по замовчуванню y - immutable argument
    y += 1;
    y
}
fn main() {
    let mut x = 42;
    take_by_reference(&mut x);
    let x = 42;
    let x = match take_by_value(x) {
       value if value >= 0 => value,
       _ => x
    };
    println!("x: {}, y: {}", x, y);
}
