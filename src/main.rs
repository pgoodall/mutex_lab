use std::thread;
use std::sync::Mutex;

fn main() {
    let mut data = Mutex::new(vec![1, 2, 3]);

    for i in 0..3 {
        // Try to capture a mutable reference in multiple threads
        // This will fail to compile!
        let handle = thread::spawn(move || {
            let mut value = data.lock().unwrap();
            value[i] += 1;
        });

        handle.join().unwrap();

    }
}
