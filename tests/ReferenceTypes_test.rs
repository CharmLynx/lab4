fn take_by_reference(x: &mut i32) {
    *x += 1;
}

fn main() {
    let mut x = 42;
    take_by_reference(&mut x);
    println!("x: {}", x); // x: 43
}

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
