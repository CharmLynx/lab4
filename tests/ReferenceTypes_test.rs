fn take_by_reference(reference: &mut i32) {
    *reference += 10;
}

fn main() {
    let mut reference = &mut 10;
    take_by_reference(reference);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reference_is_modified() {
        let mut x = 10;
        take_by_reference(&mut x);
        assert_eq!(x, 20);
    }
}
