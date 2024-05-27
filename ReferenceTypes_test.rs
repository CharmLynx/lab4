fn take_by_reference(_: &i32) {}

fn main() {
    let x = 42;
    take_by_reference(&x);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reference_is_passed() {
        let x = 42;
        let x_ptr_before = &x as *const i32;
        take_by_reference(&x);
        let x_ptr_after = &x as *const i32;
        assert_eq!(x_ptr_before, x_ptr_after);
    }
}
