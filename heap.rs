fn main() {

    struct Point {
        x: i32,
        y: i32,
    }

    let p = Box::new(Point { x: 10, y: 20 });

    println!("Point coordinates: ({}, {})", p.x, p.y);
}