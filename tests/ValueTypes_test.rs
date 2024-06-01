fn take_by_value(x: i32) -> i32 {
    x += 1;
    x
}

fn main() {
    let x = 42;
    let x = take_by_value(x);
    println!("x: {}, y: {}", x, y);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_is_copied() {
        let x = 42;
        let x = match take_by_value(x) {
            value if value >= 0 => value,
            _ => x
        };
        assert_eq!(x, 42);
    }
}
