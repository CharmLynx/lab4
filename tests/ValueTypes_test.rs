fn take_by_value(mut value: i32) {
    value += 13;
}

fn main() {
    let value = 5;
    take_by_value(value);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
     fn test_value_isnt_modified() {
        let x = 42;
        take_by_value(x);
        assert_eq!(x, 42);
    }
}
