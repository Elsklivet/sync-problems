use std::thread;
use std::time::Duration;
use std::sync::Condvar;
use std::sync::Mutex;
use std::sync::MutexGuard;
use std::sync::Arc;
use std::env;

const NUM_READERS: i32 = 5;
const NUM_WRITERS: i32 = 2;

struct Shared {
    writers_inside: i32,
    readers_inside: i32,
    data: i32
}

fn reader_spawner(shared: Arc<Mutex<Shared>>, condvars: Arc<(Condvar,Condvar)>) {
    let mut threads = vec![];

    for i in 0..NUM_READERS {

        threads.push(thread::spawn(move || {
            // thread::sleep(Duration::from_millis(100));
            println!("Reader {} arrives",i);

        }));
    }

    for t in threads {
        let _ = t.join().unwrap();
    }
}

fn writer_spawner(shared: Arc<Mutex<Shared>>, condvars: Arc<(Condvar,Condvar)>) {
    let mut threads = vec![];

    for i in 0..NUM_WRITERS {
        
        threads.push(thread::spawn(move || {
            // thread::sleep(Duration::from_millis(100));
            println!("Writer {} arrives",i);

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
