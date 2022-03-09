use std::thread;
use std::time::Duration;
use std::sync::Condvar;
use std::sync::Mutex;
use std::sync::MutexGuard;
use std::sync::Arc;

const NUM_PRODUCERS: i32 = 3;
const NUM_CONSUMERS: i32 = 100;

fn main() {
    // Track number of threads
    let mut threads = vec![];

    let triple = Arc::new((Mutex::new(false), Mutex::new(vec![0]), Condvar::new()));

    for i in 0..NUM_CONSUMERS {
        let c_triple = Arc::clone(&triple);
        // Consumers spawner
        threads.push(thread::spawn(move || {
            // Consumer code
            println!("consumer {} starts", i);

            let (buff_empty_lock, buff_lock, item_produced) = &*c_triple;

            let mut buff_empty = buff_empty_lock.lock().unwrap();

            while *buff_empty { 
                println!("consumer {} waits for buffer to not be empty",i);
                buff_empty = item_produced.wait(buff_empty).unwrap();
            }

            // Now we should know the buffer is NOT empty

            // Consume and print
            let item = buff_lock.lock().unwrap().pop();
            *buff_empty = buff_lock.lock().unwrap().len() == 0;

            println!("consumer {} consumed {:?} from buffer",i,item);

            println!("consumer {} exits", i);
        }));

        // thread::sleep(Duration::from_secs(1));
    }

    for j in 0..NUM_PRODUCERS {
        let c_triple = Arc::clone(&triple);
        // Producer spawner
        threads.push(thread::spawn(move || {
            // Producer code
            println!("producer {} starts", j);

            let (buff_empty_lock, buff_lock, item_produced) = &*c_triple;

            let mut buff_empty = buff_empty_lock.lock().unwrap();

            for _ in 0..NUM_CONSUMERS/NUM_PRODUCERS+1 {
                // Wastefully large prodduction right now
                println!("producer {} produces 0 to the buffer",j);
                buff_lock.lock().unwrap().push(0);

                *buff_empty = false;
                println!("producer {} notifies all consumers",j);
                item_produced.notify_all();
            }

            println!("producer {} exits", j);
        }));

        // thread::sleep(Duration::from_secs(1));
    }

    for t in threads {
        let _ = t.join().unwrap();
    }
}