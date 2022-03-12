# Gavin Heinrichs

# 1. Any number of readers can be in the critical section simultaneously.
# 2. Writers must have exclusive access to the critical section.

# In other words, a writer cannot enter the critical section while any other
# thread (reader or writer) is there, and while the writer is there, no other thread
# may enter (Downey, p. 65).

import threading
import random
from sys import argv

NUM_READERS = 5
NUM_WRITERS = 1

# Shared data
shared_data = "abcde"
readers_inside = 0
writers_inside = 0
writer_can_enter = threading.Condition()
reader_can_enter = threading.Condition()
lock = threading.Lock()

def Reader(id):
    global readers_inside
    global shared_data
    global writers_inside

    print(f"Reader {id} arrives")
    lock.acquire()
    # Reader waits to enter if a writer is in the room
    while writers_inside > 0:
        with reader_can_enter:
            print(f"Reader {id} waits to enter, since {writers_inside} writers are inside")
            lock.release()
            reader_can_enter.wait()
            lock.acquire()
            print(f"Reader {id} wakes up to enter")

    # Reader safe to enter
    print(f"Reader {id} enters the room")
    readers_inside += 1

    # Reader should be able to read now
    print(f"Reader {id} reads {shared_data} from the shared data buffer")  

    # Reader goes home
    print(f"Reader {id} leaves")
    readers_inside -= 1

    if readers_inside == 0:
        with writer_can_enter:
            writer_can_enter.notify_all()

    lock.release()
    
    


def Writer(id):
    global writers_inside
    global shared_data
    global readers_inside

    print(f"Writer {id} arrives")

    # Writer checks if readers or writers are in the room
    lock.acquire()
    while writers_inside > 0 or readers_inside > 0:
        with writer_can_enter:
            print(f"Writer {id} waits to enter")
            lock.release()
            writer_can_enter.wait()
            lock.acquire()
            print(f"Writer {id} wakes up to enter")
    
    # Writer safe to enter
    print(f"Writer {id} enters")
    writers_inside += 1

    # Writer can write to data
    shared_data = shared_data + shared_data
    print(f"Writer {id} writes {shared_data} to shared data")

    # Writer leaves
    print(f"Writer {id} leaves")
    writers_inside -= 1

    with writer_can_enter:
        writer_can_enter.notify_all()

    with reader_can_enter:
        reader_can_enter.notify_all()

    lock.release()

if __name__=="__main__":
    if len(argv) == 3:
        try:
            NUM_READERS = int(argv[1])
            NUM_WRITERS = int(argv[2])
        except TypeError:
            print("Usage: python3.10 readers-writers.py <number of reader threads: int>? <number of writer threads: int>?")

    threads = []
    
    for i in range(NUM_READERS):
        ri = threading.Thread(target=Reader, args=(i,))
        threads.append(ri)

    for i in range(NUM_WRITERS):
        wi = threading.Thread(target=Writer, args=(i,))
        threads.append(wi)

    random.shuffle(threads)
    for t in threads:
        t.start()
            
    # Cleanup
    parent = threading.current_thread()
    for thread in threading.enumerate():
        if thread is not parent:
            thread.join()

    print(f"Shared data is {shared_data} at end. There are {readers_inside} readers and {writers_inside} writers inside.")