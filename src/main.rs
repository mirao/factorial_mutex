use std::sync::{Arc, Mutex};
use std::thread;

/// Factorial result
struct Factorial {
    num: u8,
    result: u64,
}

/// Get factorial for given number
fn factorial(num: u8) -> u64 {
    if num > 0 {
        num as u64 * factorial(num - 1)
    } else {
        1
    }
}

fn main() {
    // Vector with factorial results
    // 'Arc' allows sharing of vector between threads, `Mutex` allows pushing of factorial into vector
    let factorials = Arc::new(Mutex::new(Vec::new()));
    // Thread handles
    let mut handles = vec![];

    // Compute factorials 0..19, one per thread
    for num in 0..20 {
        let factorials_copy = Arc::clone(&factorials);
        let handle = thread::spawn(move || {
            factorials_copy.lock().unwrap().push(Factorial {
                num,
                result: factorial(num),
            })
        });
        handles.push(handle);
    }

    // Wait for finishing of all threads
    for handle in handles {
        handle.join().unwrap();
    }

    // Sort factorials by orig number
    let mut factorials_sorted = factorials.lock().unwrap();
    factorials_sorted.sort_by_key(|k| k.num);

    // Print factorials
    for factorial in factorials_sorted.iter() {
        println!("{}! = {}", factorial.num, factorial.result);
    }
}
