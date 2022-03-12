import threading
import time
import random
import sys

buffer = [] # Items buffer (just a list)
buffer_lock = threading.Lock()
item_produced = threading.Condition()

num_consumers = 100
num_producers = 3

def Producer(id):
    global buffer
    global buffer_lock
    global item_produced
    # Producer pseudo (from textbook):
    # event = waitForEvent()
    # buffer.add(event)
    print(f"producer {id} starts")
    for _ in range((num_consumers//3)+1): # wastefully large right now, maybe only produce when it becomes empty?
        added = random.randint(0,1)
        with buffer_lock:
            print(f"producer {id} adds {added} to the buffer")
            buffer.append(added)
        with item_produced:
            print(f"producer {id} notifies consumers they can consume")
            item_produced.notify_all()
            
    print(f"producer {id} exits")
        
    

def Consumer(id):
    global buffer
    global buffer_lock
    global item_produced
    # Consumer pseudo (from textbook):
    # event = buffer.pop()
    # event.process() (do something with event)
    print(f"consumer {id} starts")
    
    with item_produced:
        print(f"consumer {id} starts waiting loop")
        
        
        with buffer_lock:
            while len(buffer) == 0:
                buffer_lock.release()
                print(f"consumer {id} waits for an item to be produced")
                item_produced.wait()
                print(f"consumer {id} returns from waiting")
                buffer_lock.acquire()
            
            item = buffer.pop()
            print(f"consumer {id} reads {item} from buffer")
    
    print(f"consumer {id} exits")
    


if __name__=="__main__":
    for i in range(num_consumers):
        ci = threading.Thread(target=Consumer, args=(i,))
        ci.start()

    for i in range(num_producers):
        pi = threading.Thread(target=Producer, args=(i,))
        pi.start()
            
    # Cleanup
    parent = threading.current_thread()
    for thread in threading.enumerate():
        if thread is not parent:
            thread.join()