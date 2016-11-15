struct Point {
    x: isize,
    y: isize,
}

fn main() {
    let a = Point { x: 10, y: 20 };
    let b = a;
    println!("{}", a.x);
}

