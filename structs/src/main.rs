
struct Point {
    x: i32,
    y: i32
}
fn main() {
    let mut p = Point { x: 1, y: 2 };
    let x = &mut p.x;
    let y = &mut p.y;
    *x += 1;
    *y += 1;
    println!("{} {}", p.x, p.y);
}