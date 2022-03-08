import threading
import time
import random
import sys

buffer = [] # Items buffer (just a list)
buffer_lock = threading.Lock()
item_produced = threading.Condition()

def Producer(id):
    global buffer
    global buffer_lock
    global item_produced
    # Producer pseudo (from textbook):
    # event = waitForEvent()
    # buffer.add(event)
    print(f"producer {id} starts")
    for _ in range(3):
        added = random.randint(0,1)
        with buffer_lock:
            print(f"producer {id} adds {added} to the buffer")
            buffer.append(added)
        with item_produced:
            print(f"producer {id} notifies consumers they can consume")
            item_produced.notify_all()
        
    

def Consumer(id):
    global buffer
    global buffer_lock
    global item_produced
    # Consumer pseudo (from textbook):
    # event = buffer.pop()
    # event.process() (do something with event)
    print(f"consumer {id} starts")
    
    with item_produced:
        print(f"consumer {id} starts waiting for an item to be produced")
        
        
        with buffer_lock:
            while len(buffer) == 0:
                buffer_lock.release()
                item_produced.wait()
                print(f"consumer {id} returns from waiting")
                buffer_lock.acquire()
            
            item = buffer.pop()
            print(f"consumer {id} reads {item} from buffer")
    


if __name__=="__main__":
    # Threads must have exclusive access to buffer
    # If a consumer arrives while the buffer is empty, it blocks until
    # a producer has filled the thread
    
    for i in range(3):
        ci = threading.Thread(target=Consumer, args=(i,))
        ci.start()

    pi = threading.Thread(target=Producer, args=(0,))
    pi.start()
    
    # Cleanup
    parent = threading.current_thread()
    for thread in threading.enumerate():
        if thread is not parent:
            thread.join()