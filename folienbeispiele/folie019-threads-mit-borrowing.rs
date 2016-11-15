fn worker(value: Arc<Mutex<i32>>) {
    let mut val = value.lock().unwrap();
    println!("Working on {}", *val);
    *val = *val + 1;
}

fn main() {
    let value = Arc::new(Mutex::new(1));

    for i in 0..3 {
        let value = value.clone();
        thread::spawn(move || {
            worker(value)
        });
    }
}

