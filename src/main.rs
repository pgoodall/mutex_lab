use std::thread;
use std::sync::Mutex;

fn main() {
    let mut data = Mutex::new(vec![1, 2, 3]);

    // for i in 0..3 {
        // Try to capture a mutable reference in multiple threads
        // This will fail to compile!
    let handle = thread::spawn(move || {
        for i in 0..3 {
            let mut value = data.lock().unwrap();
            value[i] += 1;
        }
        data
    });

    match handle.join() {
        Ok(result) => println!("Thread finished with result: {:?}", result),
        Err(e) => println!("Thread panicked: {:?}", e),
    }

    //println!("Data: {:?}", data.lock().unwrap());
}
