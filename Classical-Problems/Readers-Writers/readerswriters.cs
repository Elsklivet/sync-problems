using System;
using System.Threading;
using System.Collections.Concurrent;
using System.Collections.Generic;

public class ReadersWritersClass {

    // Shared variable hours
    internal int writersInside = 0;
    internal int readersInside = 0;
    internal int data = 0;

    // Time for sync primitives
    internal Mutex mutex;
    internal AutoResetEvent writerCanEnter;
    internal ManualResetEvent readerCanEnter;


    internal class Reader {
        private int id;
        private ReadersWritersClass upper;
        public Reader(int i, ReadersWritersClass upper) {
            this.id = i;
            this.upper = upper;
        }

        public void run() {
            Console.WriteLine($"Reader {id} arrives");

            upper.mutex.WaitOne();
            // Need to check if a writer is in the room
            while ( upper.writersInside > 0 ) {
                upper.mutex.ReleaseMutex();
                upper.readerCanEnter.WaitOne();
                upper.mutex.WaitOne();
            }
            Console.WriteLine($"Reader {id} enters the room");
            upper.readersInside++;
            upper.mutex.ReleaseMutex();

            upper.mutex.WaitOne();
            Console.WriteLine($"Reader {id} reads {upper.data} from shared data");
            upper.mutex.ReleaseMutex();

            upper.mutex.WaitOne();
            Console.WriteLine($"Reader {id} leaves the room");
            upper.readersInside--;
            if ( upper.readersInside == 0 ) {
                upper.mutex.ReleaseMutex();
                upper.writerCanEnter.Set();
                return;
            }
            upper.mutex.ReleaseMutex();
        }
    }

    internal class Writer {
        private int id;
        private ReadersWritersClass upper;
        public Writer(int i, ReadersWritersClass upper) {
            this.id = i;
            this.upper = upper;
        }
        public void run() {
            Console.WriteLine($"Writer {id} arrives");

            upper.mutex.WaitOne();
            while ( upper.writersInside > 0 || upper.readersInside > 0 ) {
                upper.mutex.ReleaseMutex();
                upper.writerCanEnter.WaitOne();
                upper.mutex.WaitOne();
            }
            Console.WriteLine($"Writer {id} enters the room");
            upper.writersInside++;
            upper.writerCanEnter.Reset();
            upper.mutex.ReleaseMutex();

            upper.mutex.WaitOne();
            upper.data++;
            Console.WriteLine($"Writer {id} wrote {upper.data} to shared data");
            upper.mutex.ReleaseMutex();

            upper.mutex.WaitOne();
            Console.WriteLine($"Writer {id} leaves the room");
            upper.writersInside--;
            upper.readerCanEnter.Set();
            upper.writerCanEnter.Set();
            upper.mutex.ReleaseMutex();
        }
    }

    internal class Spawners {
        public Spawners(){
            // do nothing
        }
        public void ReaderSpawner(int numReaders, ReadersWritersClass upper) {
            Thread[] threads = new Thread[numReaders];

            // Spawn given number of threads
            for (int i = 0; i < numReaders; i++) {
                Reader r = new Reader(i, upper);
                Thread t = new Thread(new ThreadStart(r.run));
                t.Start();
                threads[i] = t;
            }


            // Join all threads we made
            foreach (var t in threads)
            {
                t.Join();
            }
        }

        public void WriterSpawner(int numWriters, ReadersWritersClass upper) {
            // This might need to be atomic so it doesn't get starved by reader spawner
            Thread[] threads = new Thread[numWriters];

            // Spawn given number of threads
            for (int i = 0; i < numWriters; i++) {
                Writer w = new Writer(i, upper);
                Thread t = new Thread(new ThreadStart(w.run));
                t.Start();
                threads[i] = t;
            }


            // Join all threads we made
            foreach (var t in threads)
            {
                t.Join();
            }
        }
    }

    ReadersWritersClass(int r, int w) {
        this.mutex = new Mutex();
        this.writerCanEnter = new AutoResetEvent(false);
        this.readerCanEnter = new ManualResetEvent(true);
        this.data = 0;
        this.writersInside = 0;
        this.readersInside = 0;

        Spawners spawner = new Spawners();

        ThreadStart readerTS = delegate {
            spawner.ReaderSpawner(r, this);
        };

        ThreadStart writerTS = delegate {
            spawner.WriterSpawner(w, this);
        };
        Thread readerSpawner = new Thread(readerTS);
        Thread writerSpawner = new Thread(writerTS);

        readerSpawner.Start();
        writerSpawner.Start();

        readerSpawner.Join();
        writerSpawner.Join();
    }
    public static void Main(string[] args) {
        int r = 3, w = 1;
        // Use command line args soon
        if ( args.Length >= 2 ) {
            r = int.Parse(args[0]);
            w = int.Parse(args[1]);
        }
        new ReadersWritersClass(r,w);
    }
}