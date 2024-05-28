fn take_by_value(_: i32) {}

fn main() {
    let x = 42;
    take_by_value(x);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_is_copied() {
        let x = 42;
        let x_before = x;
        take_by_value(x);
        assert_eq!(x, x_before);
    }
}
