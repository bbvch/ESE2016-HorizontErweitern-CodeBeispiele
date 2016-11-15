use std::thread;

fn worker(val: &mut i32) {
    println!("Working on {}", val);
    *val = *val + 1;
}

fn main() {
    let mut value = 1;

    for i in 0..3 {
        thread::spawn(|| {
            worker(&mut value)
        });
    }
}

