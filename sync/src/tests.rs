use std::sync::{Arc, Mutex, RwLock};
use std::thread;

#[test]
fn test_mutex_example() {
    // Create a shared counter protected by a Mutex
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    // Spawn 10 threads that each increment the counter
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Verify the final count
    let result = counter.lock().unwrap();
    assert_eq!(*result, 10);
}

#[test]
fn test_rwlock_example() {
    // Create a shared value protected by an RwLock
    let data = Arc::new(RwLock::new(String::from("Hello")));
    let mut handles = vec![];

    // Spawn multiple reader threads
    for i in 0..5 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let reader = data.read().unwrap();
            println!("Reader {}: {}", i, *reader);
        });
        handles.push(handle);
    }

    // Spawn a writer thread
    let data_writer = Arc::clone(&data);
    let writer_handle = thread::spawn(move || {
        let mut writer = data_writer.write().unwrap();
        writer.push_str(", World!");
    });

    handles.push(writer_handle);

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Verify the final value
    let result = data.read().unwrap();
    assert_eq!(*result, "Hello, World!");
}
