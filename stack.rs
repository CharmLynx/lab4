fn main() {
    
    let x: i32 = 5;

    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 10, y: 20 };

    println!("Point coordinates: ({}, {})", p.x, p.y);
}