using System;
using System.Threading;
using System.Collections.Concurrent;
using System.Collections.Generic;

public class Reader {
    private int id;
    public Reader(int i) {
        this.id = i;
    }

    public void run() {
        Console.WriteLine($"Reader {id} arrives");
    }
}

public class Writer {
    private int id;
    public Writer(int i) {
        this.id = i;
    }
    public void run() {
        Console.WriteLine($"Writer {id} arrives");
    }
}

public class Spawners {
    public static void ReaderSpawner(int numReaders) {
        Thread[] threads = new Thread[numReaders];

        // Spawn given number of threads
        for (int i = 0; i < numReaders; i++) {
            Reader r = new Reader(i);
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

    public static void WriterSpawner(int numWriters) {
        // This might need to be atomic so it doesn't get starved by reader spawner
        Thread[] threads = new Thread[numWriters];

        // Spawn given number of threads
        for (int i = 0; i < numWriters; i++) {
            Writer w = new Writer(i);
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
public class ReadersWritersClass {
    ReadersWritersClass(int r, int w) {
        ThreadStart readerTS = delegate {
            Spawners.ReaderSpawner(r);
        };

        ThreadStart writerTS = delegate {
            Spawners.WriterSpawner(w);
        };
        Thread readerSpawner = new Thread(readerTS);
        Thread writerSpawner = new Thread(writerTS);

        readerSpawner.Start();
        writerSpawner.Start();

        readerSpawner.Join();
        writerSpawner.Join();
    }
    public static void Main(string[] args) {
        // Use command line args soon
        new ReadersWritersClass(3,1);
    }
}