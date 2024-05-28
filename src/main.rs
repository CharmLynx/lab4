fn take_by_reference(_: &i32) {}

fn take_by_value(_: i32) {}

fn main() {
    let x = 42;
    take_by_reference(&x);
    take_by_value(x);
}
