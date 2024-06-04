fn take_by_reference(reference: &mut i32) {
    *reference += 10;
}

fn main() {
    let mut reference = &mut 10;
    take_by_reference(&mut reference);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reference_is_modified() {
       let mut reference = 10;
       take_by_reference(&mut reference);
       assert_eq!(reference,20)
}

}
