#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_is_copied() {
        let x = 42;
        let y = take_by_value(x);
        assert_eq!(x, 42);
        assert_eq!(y, 43);
    }
}
