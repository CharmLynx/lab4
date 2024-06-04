fn take_by_value(mut value: i32) {
    value += 13;
}

fn main() {
    let mut value = 5;
    take_by_value(value);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_isnt_modified() {
        let mut value = 5;
        take_by_value(value);
        assert_eq!(value, 5);
    }
}
