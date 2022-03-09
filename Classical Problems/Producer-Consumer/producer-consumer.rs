use std::thread;
use std::time::Duration;
use std::sync::Condvar;
use std::sync::Mutex;
use std::sync::MutexGuard;
use std::sync::Arc;

const NUM_PRODUCERS: i32 = 1;
const NUM_CONSUMERS: i32 = 10;

fn main() {
    // Track number of threads
    let mut threads = vec![];

    // Need a shared buffer
    let buffer = Arc::new(Mutex::new(vec![0]));

    // Item production condition variable
    let item_produced = Arc::new(Condvar::new());

    for i in 0..NUM_CONSUMERS {
        let cloned_buffer = buffer.clone();
        let cloned_item_produced = item_produced.clone();
        // Consumers spawner
        threads.push(thread::spawn(move || {
            // Consumer code
            println!("consumer {} starts", i);
            
            // Lock buffer and wait until consumable
            let buff_guard = cloned_buffer.lock().unwrap();
            // How do I actually surround this with a loop because of all these unwraps?
            //while buff_guard.len() == 0 { // <-- does not work
            let item = cloned_item_produced.wait(buff_guard).unwrap().pop();
            println!("consumer {} is done waiting, read {:?}",i,item);
            //}


            println!("consumer {} exits", i);
        }));
    }

    for j in 0..NUM_PRODUCERS {
        let cloned_buffer = buffer.clone();
        let cloned_item_produced = item_produced.clone();
        // Producer spawner
        threads.push(thread::spawn(move || {
            // Producer code
            println!("producer {} starts", j);

            for _ in 0..NUM_CONSUMERS {
                println!("producer {} relocks and adds to buffer",j);

                cloned_buffer.lock().unwrap().push(0);
                // Unlock?
                
                println!("producer {} notifies all consumers",j);
                // Notify thread that it can consume
                cloned_item_produced.notify_all();
            }

            println!("producer {} exits", j);
        }));
    }

    for t in threads {
        let _ = t.join().unwrap();
    }
}