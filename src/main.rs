use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
    // Start: My working code
    // let mut data = vec![1, 2, 3];

    // let handle = thread::spawn(move || {
    //     for value in &mut data {
    //         *value += 1;
    //     }

    //     data
    // });

    // match handle.join() {
    //     Ok(d) => println!("Output: {:?}", d),
    //     Err(e) => eprint!("Error: {:?}", e),
    // };
    // End: My working code
    
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));
    let mut handles = vec![];

    for i in 0..3 {
        // Try to capture a mutable reference in multiple threads
        // This will fail to compile!
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let mut value = data.lock().unwrap();
            value[i] += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Output: {:?}", data.lock().unwrap());
}
