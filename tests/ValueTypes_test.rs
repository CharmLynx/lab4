fn take_by_value(mut x: i32) -> i32 {
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
        let y = take_by_value(x);
        assert_eq!(x, 42);
        //assert_eq!(y, 43);
    }
}
