import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.locks.Condition;
import java.util.concurrent.locks.ReentrantLock;

public class readerswriters {

    private static ReentrantLock lock;
    private static Condition writerCanEnter; 
    private static Condition readerCanEnter;
    private static int writersInside;
    private static int readersInside;
    private static Integer data;

    public class Reader implements Runnable {
        private int id;

        public Reader(int id){
            this.id = id;
        }

        public void run() {
            System.err.printf("Reader %d arrives\n",id);
            synchronized (lock) {
                while (writersInside > 0) {
                    try {
                        System.err.printf("Reader %d waits to enter\n",id);
                        readerCanEnter.wait();
                    } catch (InterruptedException iex) {
                        System.err.printf("Reader %d interrupted in wait call\nStacktrace:",id);
                        iex.printStackTrace();
                    }
                }
                
                System.err.printf("Reader %d enters the room\n",id);
                readersInside++;
            }
            // Break so other readers can enter 
            synchronized (lock) {
                System.err.printf("Reader %d reads %d from the shared data\n",id,data.intValue());
                
                System.err.printf("Reader %d leaves the room\n",id);
                readersInside--;

                if(readersInside == 0) {
                    System.err.printf("Reader %d notifies all writers to enter\n",id);
                    writerCanEnter.notifyAll();
                }
            }

            System.err.printf("Reader %d thread exits\n",id);
        }
    }

    public class Writer implements Runnable {
        private int id;

        public Writer(int id){
            this.id = id;
        }
        
        public void run() {
            System.err.printf("Writer %d arrives\n",id);

            synchronized (lock) {
                while ( writersInside > 0 || readersInside > 0 ) {
                    try {
                        System.err.printf("Writer %d waits to enter\n",id);
                        writerCanEnter.wait();
                    } catch (InterruptedException iex) {
                        System.err.printf("Writer %d interrupted in wait call\nStacktrace:",id);
                        iex.printStackTrace();
                    }
                }

                System.err.printf("Writer %d enters the room\n",id);
                writersInside++;
            }
            // Break to allow interrupts
            synchronized (lock) {
                data = Integer.valueOf(data.intValue()+1);
                System.err.printf("Writer %d writes %d to shared data\n",id,data.intValue());

                System.err.printf("Writer %d leaves the room\n",id);
                writersInside--;

                System.err.printf("Writer %d notifies all threads to enter\n",id);
                writerCanEnter.notifyAll();
                readerCanEnter.notifyAll();
            }

            System.err.printf("Writer %d thread exits\n",id);
        }
    }

    public class WriterSpawner implements Runnable {

        private final ExecutorService pool;
        private final int NUM_WRITERS;

        public WriterSpawner(int n){
            this.pool = Executors.newCachedThreadPool();
            this.NUM_WRITERS = n;
        }

        public void run() {
            for ( int i = 0; i < this.NUM_WRITERS; i++ ) {
                pool.submit(new Writer(i));
            }

            pool.shutdown();
        }

    }

    public class ReaderSpawner implements Runnable {

        private final ExecutorService pool;
        private final int NUM_READERS;

        public ReaderSpawner(int n){
            this.pool = Executors.newCachedThreadPool();
            this.NUM_READERS = n;
        }

        public void run() {
            for ( int i = 0; i < this.NUM_READERS; i++ ) {
                pool.submit(new Reader(i));
            }

            pool.shutdown();
        }

    }

    public readerswriters(int readers, int writers){
        // Use futures instead:
        // https://stackoverflow.com/questions/20495414/thread-join-equivalent-in-executor
        data = Integer.valueOf(0);
        lock = new ReentrantLock();
        writerCanEnter = lock.newCondition();
        readerCanEnter = lock.newCondition();
        ExecutorService spawnerPool = Executors.newFixedThreadPool(2);

        spawnerPool.submit(new ReaderSpawner(readers));
        spawnerPool.submit(new WriterSpawner(writers));

        spawnerPool.shutdown();
    }

    public static void main(String[] args) {
        int r = 3, w = 1;
        if (args.length == 2) {
            // Given num readers/writers as arguments
            try{
                r = Integer.parseInt(args[0]);
                w = Integer.parseInt(args[1]);
            } catch (NumberFormatException nfex) {
                System.err.println("Usage: java readerswriters <num readers: int> <num writers: int>");
            }
        }
        new readerswriters(r,w);
    }
}