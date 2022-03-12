use std::thread;
use std::time::Duration;
use std::sync::Condvar;
use std::sync::Mutex;
use std::sync::MutexGuard;
use std::sync::Arc;
use std::env;

const NUM_READERS: i32 = 5;
const NUM_WRITERS: i32 = 2;

fn reader_spawner(data: Arc<Mutex<i32>>, inside_pair: Arc<(Mutex<i32>, Mutex<i32>)>, condvars: Arc<(Condvar,Condvar)>, predicates: Arc<(Mutex<bool>,Mutex<bool>)>) {
    let mut threads = vec![];

    for i in 0..NUM_READERS {
        let c_data = Arc::clone(&data);
        let c_inside = Arc::clone(&inside_pair);
        let c_condvars = Arc::clone(&condvars);
        let c_preds = Arc::clone(&predicates);

        threads.push(thread::spawn(move || {
            // thread::sleep(Duration::from_millis(100));
            println!("Reader {} arrives",i);

            let (reader_ok_l, writer_ok_l) = &*c_preds;
            let (readers_inside_l, writers_inside_l) = &*c_inside;
            let (reader_can_enter, writer_can_enter) = &*c_condvars;
            
            println!("Reader {} attempts to lock reader_ok_l",i);
            let mut reader_ok = reader_ok_l.lock().unwrap();
            println!("Reader {} successfully locked reader_ok_l",i);

            // Can enter if no writers are in the room
            while !*reader_ok {
                println!("Reader {} should wait until all writers leave",i);
                reader_ok = reader_can_enter.wait(reader_ok).unwrap();
            }  

            println!("Reader {} attempts to lock writer_ok_l",i);
            let mut writer_ok = writer_ok_l.lock().unwrap();
            println!("Reader {} successfully locked writer_ok_l",i);
            *writer_ok = false;


            println!("Reader {} enters safely",i);

            let mut readers_inside = readers_inside_l.lock().unwrap();
            *readers_inside += 1;

            // Read data
            let data_guard = (*c_data).lock();
            let read = data_guard.unwrap();
            println!("Reader {} reads {:?} from data",i,read);
            
            println!("Reader {} leaves",i);
            *readers_inside -= 1;

            if *readers_inside == 0 {
                *writer_ok = true;
                writer_can_enter.notify_all();
            }
        }));
    }

    for t in threads {
        let _ = t.join().unwrap();
    }
}

fn writer_spawner(data: Arc<Mutex<i32>>, inside_pair: Arc<(Mutex<i32>, Mutex<i32>)>, condvars: Arc<(Condvar,Condvar)>, predicates: Arc<(Mutex<bool>,Mutex<bool>)>) {
    let mut threads = vec![];

    for i in 0..NUM_WRITERS {
        let c_data = Arc::clone(&data);
        let c_inside = Arc::clone(&inside_pair);
        let c_condvars = Arc::clone(&condvars);
        let c_preds = Arc::clone(&predicates);
        
        threads.push(thread::spawn(move || {
            // thread::sleep(Duration::from_millis(100));
            println!("Writer {} arrives",i);

            let (reader_ok_l, writer_ok_l) = &*c_preds;
            let (readers_inside_l, writers_inside_l) = &*c_inside;
            let (reader_can_enter, writer_can_enter) = &*c_condvars;

            println!("Writer {} attempts to lock writer_ok_l",i);
            let mut writer_ok = writer_ok_l.lock().unwrap();
            println!("Writer {} successfully locks writer_ok_l",i);

            while !*writer_ok {
                println!("Writer {} should wait until the room is empty",i);
                writer_ok = writer_can_enter.wait(writer_ok).unwrap();
            }
            // Cannot drop, another use of this later
            *writer_ok = false;
            println!("Writer {} attempts to lock reader_ok_l",i);
            let mut reader_ok = reader_ok_l.lock().unwrap();
            println!("Writer {} successfully locked reader_ok_l",i);
            *reader_ok = false;
            
            let mut writers_inside = writers_inside_l.lock().unwrap();
            println!("Writer {} safely enters",i);
            *writers_inside += 1;

            let data_guard = (*c_data).lock();
            let mut writes = data_guard.unwrap();
            *writes += 1;
            println!("Writer {} writes {:?} to data",i,writes);

            println!("Writer {} leaves",i);
            *writers_inside -= 1;

            if *writers_inside == 0 {
                *reader_ok = true;
                *writer_ok = true;
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

    let data = Arc::new(Mutex::new(0));
    let inside_pair = Arc::new((Mutex::new(0), Mutex::new(0))); // readers inside, writers inside
    let condvars = Arc::new((Condvar::new(), Condvar::new())); // reader_can_enter, writer_can_enter
    let predicates = Arc::new((Mutex::new(true), Mutex::new(true))); // reader ok to enter, writer ok to enter

    // Need a data var, will be an int
    // Need a lock on that var
    // Need a writers_inside and readers_inside, all locked
    // Need two condvars, reader_can_enter and writer_can_enter
    // Need two predicates, one for each of those

    let c1_data = Arc::clone(&data);
    let c1_inside = Arc::clone(&inside_pair);
    let c1_condvars = Arc::clone(&condvars);
    let c1_preds = Arc::clone(&predicates);
    threads.push(thread::spawn(move || {
        reader_spawner(c1_data, c1_inside, c1_condvars, c1_preds);
    }));

    let c2_data = Arc::clone(&data);
    let c2_inside = Arc::clone(&inside_pair);
    let c2_condvars = Arc::clone(&condvars);
    let c2_preds = Arc::clone(&predicates);
    threads.push(thread::spawn(move || {
        writer_spawner(c2_data, c2_inside, c2_condvars, c2_preds);
    }));

    for t in threads {
        let _ = t.join().unwrap();
    }
}
