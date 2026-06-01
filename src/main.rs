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

    // for i in 0..3 {
        // Try to capture a mutable reference in multiple threads
        // This will fail to compile!
<<<<<<< HEAD
    let handle = thread::spawn(move || {
        for i in 0..3 {
            let mut value = data.lock().unwrap();
            value[i] += 1;
        }
        data
    });
=======
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let mut value = data.lock().unwrap();
            value[i] += 1;
        });

        handles.push(handle);
>>>>>>> d05bb6d (WIP initial commit)

    match handle.join() {
        Ok(result) => println!("Thread finished with result: {:?}", result),
        Err(e) => println!("Thread panicked: {:?}", e),
    }

<<<<<<< HEAD
    //println!("Data: {:?}", data.lock().unwrap());
=======
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Output: {:?}", data.lock().unwrap());
>>>>>>> d05bb6d (WIP initial commit)
}
