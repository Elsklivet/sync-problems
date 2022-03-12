#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#ifdef _WIN32
#include <Windows.h>
#else
#include <unistd.h>
#endif

// Shared data
struct shared_data {
    int data;
    int readers_inside;
    int writers_inside;
    pthread_mutex_t lock;
    pthread_cond_t writer_can_enter;
    pthread_cond_t reader_can_enter;
};

static struct shared_data shared; 

// Simplification definitions
#define LOCK(l) pthread_mutex_lock(l)
#define UNLOCK(l) pthread_mutex_unlock(l)
#define WAIT(cv,l) pthread_cond_wait(cv,l)
#define BROADCAST(cv) pthread_cond_broadcast(cv)

void init() {
    shared.data = 0;
    shared.readers_inside = 0;
    shared.writers_inside = 0;

    pthread_mutex_init(&shared.lock, NULL);
    pthread_cond_init(&shared.writer_can_enter, NULL);
    pthread_cond_init(&shared.reader_can_enter, NULL);
}

void destroy(){
    pthread_mutex_destroy(&shared.lock);
    pthread_cond_destroy(&shared.writer_can_enter);
    pthread_cond_destroy(&shared.reader_can_enter);
}

void reader(void* arg){
    int id = *((int*)arg);
    printf("Reader %d arrives\n",id);

    LOCK(&shared.lock); {
        while ( shared.writers_inside > 0 ) {
            // Cannot enter while writers are in the room
            // Wait on CV until it's appropriate to enter
            printf("Reader %d waits for the writers to leave\n",id);
            WAIT(&shared.reader_can_enter, &shared.lock);
            // Reacquired lock and safe to enter
        }

        printf("Reader %d enters the room\n",id);
        shared.readers_inside++;
    } UNLOCK(&shared.lock);
    // Allow for other threads to enter
    LOCK(&shared.lock); {
        
        int read = shared.data;

        printf("Reader %d reads %d from the shared data\n",id,read);

        printf("Reader %d leaves the room\n",id);
        shared.readers_inside--;

        if ( shared.readers_inside == 0 ) {
            BROADCAST(&shared.writer_can_enter);
        }
    } UNLOCK(&shared.lock);

}

void writer(void* arg){
    int id = *((int*)arg);
    printf("Writer %d arrives\n",id);
    LOCK(&shared.lock); {
        while ( shared.writers_inside > 0 || shared.readers_inside > 0 ) {
            printf("Writer %d waits for the room to empty out\n",id);
            WAIT(&shared.writer_can_enter, &shared.lock);
            // Reacquired lock
        }
        // Safe to enter
        printf("Writer %d enters the room\n",id);
        shared.writers_inside++;

        int writes = ++shared.data;
        printf("Writer %d writes %d to the shared data\n",id,writes);

        printf("Writer %d leaves\n",id);
        shared.writers_inside--;
        // Writer dominant, writers enter first
        BROADCAST(&shared.writer_can_enter);
        // Note: the above means that readers * CAN STARVE * plsfix
        BROADCAST(&shared.reader_can_enter);
    } UNLOCK(&shared.lock);
}

void* spawn_readers(void* argument) {
    int num_threads = *((int*)argument);
    pthread_t *threads = calloc(num_threads, sizeof(pthread_t));

    // Spawn
    for ( int i = 0; i < num_threads; ++i ) {
        pthread_create(&threads[i], NULL, reader, (void*)&i);
    }

    // Cleanup
    for ( int i = 0; i < num_threads; ++i ) {
        pthread_join(threads[i], NULL);
    }

    free(threads);

    return NULL;
}

void* spawn_writers(void* argument) {
    int num_threads = *((int*)argument);
    pthread_t *threads = calloc(num_threads, sizeof(pthread_t));

    // Spawn
    for ( int i = 0; i < num_threads; ++i ) {
        pthread_create(&threads[i], NULL, writer, (void*)&i);
    }

    // Cleanup
    for ( int i = 0; i < num_threads; ++i ) {
        pthread_join(threads[i], NULL);
    }

    free(threads);

    return NULL;
}

int main(int argc, char* argv[]) {

    int readers = 1, writers = 1;
    pthread_t reader_spawner, writer_spawner;

    if (argc == 3) {
        printf("Got arguments: %s %s\n", argv[1], argv[2]);
        readers = atoi(argv[1]);
        writers = atoi(argv[2]);
    }

    printf("Initializing with %d readers and %d writers.\n", readers, writers);
    // Shared initialization
    init();

    // Spawn threads
    pthread_create(&reader_spawner, NULL, spawn_readers, (void*)&readers);
    pthread_create(&writer_spawner, NULL, spawn_writers, (void*)&writers);

    pthread_join(reader_spawner, NULL);
    pthread_join(writer_spawner, NULL);

    // Shared teardown
    destroy();

}