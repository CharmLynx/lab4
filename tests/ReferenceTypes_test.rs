fn take_by_reference(reference: &mut i32) {
    *reference += 10;
}

fn main() {
    let mut reference = 10;
    take_by_reference(&mut reference);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reference_is_modified() {
        let mut x = &mut 10;
        take_by_reference(&mut x);
        assert_eq!(x,20);
    }
}
