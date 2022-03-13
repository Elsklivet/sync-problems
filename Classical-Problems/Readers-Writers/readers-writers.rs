use std::thread;
use std::time::Duration;
use std::sync::Condvar;
use std::sync::Mutex;
use std::sync::MutexGuard;
use std::sync::Arc;
use std::env;

const NUM_READERS: i32 = 10;
const NUM_WRITERS: i32 = 3;

struct Shared {
    writers_inside: i32,
    readers_inside: i32,
    data: i32
}

fn reader_spawner(shared: Arc<Mutex<Shared>>, condvars: Arc<(Condvar,Condvar)>) {
    let mut threads = vec![];

    for i in 0..NUM_READERS {
        let c_shared = Arc::clone(&shared);
        let c_condvars = Arc::clone(&condvars);
        threads.push(thread::spawn(move || {
            // thread::sleep(Duration::from_millis(100));
            println!("Reader {} arrives",i);

            let (reader_can_enter, writer_can_enter) = &*c_condvars;
            // Effectively lock everything
            {
                let mut shared_guard = c_shared.lock().unwrap();
                while (*shared_guard).writers_inside > 0 {
                    println!("Reader {} waits for a writer to leave",i);
                    shared_guard = reader_can_enter.wait(shared_guard).unwrap();
                }
            }
            {
                let mut shared_guard = c_shared.lock().unwrap();
                println!("Reader {} enters the room",i);
                (*shared_guard).readers_inside += 1;

                println!("Reader {} reads {} from the shared data",i,(*shared_guard).data);

                println!("Reader {} leaves the room",i);
                (*shared_guard).readers_inside -= 1;
            }
            {
                let mut shared_guard = c_shared.lock().unwrap();
                if (*shared_guard).readers_inside == 0 {
                    writer_can_enter.notify_all();
                }
            }
        }));
    }

    for t in threads {
        let _ = t.join().unwrap();
    }
}

fn writer_spawner(shared: Arc<Mutex<Shared>>, condvars: Arc<(Condvar,Condvar)>) {
    let mut threads = vec![];

    for i in 0..NUM_WRITERS {
        let c_shared = Arc::clone(&shared);
        let c_condvars = Arc::clone(&condvars);
        threads.push(thread::spawn(move || {
            // thread::sleep(Duration::from_millis(100));
            println!("Writer {} arrives",i);

            let (reader_can_enter, writer_can_enter) = &*c_condvars;
            {
                let mut shared_guard = c_shared.lock().unwrap();
                while (*shared_guard).writers_inside > 0 || (*shared_guard).readers_inside > 0 {
                    println!("Writer {} waits until the room empties",i);
                    shared_guard = writer_can_enter.wait(shared_guard).unwrap();
                }
            }
            {
                let mut shared_guard = c_shared.lock().unwrap();
                println!("Writer {} entered the room",i);
                (*shared_guard).writers_inside += 1;

                (*shared_guard).data += 1;
                println!("Writer {} writes {} to shared data",i,(*shared_guard).data);

                println!("Writer {} leaves the room",i);
                (*shared_guard).writers_inside -= 1;
            }
            {
                let mut shared_guard = c_shared.lock().unwrap();
                writer_can_enter.notify_all();
                reader_can_enter.notify_all();
            }
        }));    
    }

    for t in threads {
        let _ = t.join().unwrap();
    }
}

fn main() {
    let mut threads = vec![];

    let shared = Arc::new(Mutex::new(Shared {
        writers_inside: 0,
        readers_inside: 0,
        data: 0
    }));

    // reader_caan_enter, writer_can_enter
    let condvars = Arc::new((Condvar::new(),Condvar::new()));

    let c1_shared = Arc::clone(&shared);
    let c1_condvars = Arc::clone(&condvars);
    threads.push(thread::spawn(move || {
        reader_spawner(c1_shared, c1_condvars);
    }));

    let c2_shared = Arc::clone(&shared);
    let c2_condvars = Arc::clone(&condvars);
    threads.push(thread::spawn(move || {
        writer_spawner(c2_shared, c2_condvars);
    }));

    for t in threads {
        let _ = t.join().unwrap();
    }
}
