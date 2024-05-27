fn main() {
    let mut vec = vec![1, 2, 3];
    modify_vector(&mut vec);
    println!("Vector after modification: {:?}", vec);
}

fn modify_vector(v: &mut Vec<i32>) {
    for i in v.iter_mut() {
        *i += 3;
    }
}