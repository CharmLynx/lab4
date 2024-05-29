#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reference_is_modified() {
        let mut x = 42;
        take_by_reference(&mut x);
        assert_eq!(x, 43);
    }
}
