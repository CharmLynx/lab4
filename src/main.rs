fn take_by_value(mut value: i32) {
    value += 13;
}

fn take_by_reference(reference: &mut i32) {
    *reference += 10;
}

fn main() {
    let mut value = 5;
    let mut reference = &mut 10;
    
    println!("Value type before modification: {:?}", value);
    take_by_value(value);
    println!("Value type after modification: {:?}", value);
    
    println!("Reference type before modification: {:?}", reference);
   take_by_reference(reference);
    println!("Reference type after modification: {:?}", reference);
    
}
