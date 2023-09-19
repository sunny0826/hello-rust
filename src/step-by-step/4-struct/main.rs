struct Point {
    x: f64,
    y: f64,
}

fn main() {
    let p = Point { x: 0.0, y: 0.0 };
    println!("Point is at ({}, {})", p.x, p.y);
}
