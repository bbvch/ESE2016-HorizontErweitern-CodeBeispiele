struct Point {
    x: isize,
    y: isize,
}

fn main() {
    let mut a = Point { x: 10, y: 20 };
    let b = &a;
    let c = &mut a;
    println!("{}", a.x);
    println!("{}", b.y);
}

