use std::thread;

fn worker(val: i32) -> i32 {
    println!("Working on {}", val);
    val + 1
}

fn main() {
    let join_handle = thread::spawn(|| { worker(42) } );

    println!("Worker result: {}", join_handle.join().unwrap());
}

