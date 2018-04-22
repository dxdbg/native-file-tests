
#define _GNU_SOURCE 1

#include <unistd.h>
#include <stdlib.h>
#include <pthread.h>
#include <stdio.h>
#include <assert.h>
#include <signal.h>
#include <time.h>
#include <sys/syscall.h>
#include <string.h>
#include <fcntl.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <errno.h>
#include <signal.h>

#include "bin_lib.h"

struct threadArg {
    int id;
    pthread_mutex_t *mutex;
    pthread_mutex_t *term_mutex;
};

void usr1_handle(int sig) {
    long lwp_id = udi_get_thread_id(getpid());

    bin_printf("Received %d on thread %ld\n", sig, lwp_id);

    pause();
}

void breakpoint_thr_func() {
    bin_printf("In breakpoint_thr_func\n");
}

void *entry(void *arg) {
    struct threadArg *thisArg = (struct threadArg *)arg;

    long lwp_id = udi_get_thread_id(getpid());
    bin_printf("%ld waiting on lock\n", lwp_id);

    if( pthread_mutex_lock(thisArg->mutex) != 0 ) {
        perror("pthread_mutex_lock");
        return NULL;
    }

    bin_printf("%ld obtained lock\n", lwp_id);

    if( pthread_mutex_unlock(thisArg->mutex) != 0 ) {
        perror("pthread_mutex_unlock");
        return NULL;
    }

    bin_printf("%ld released lock\n", lwp_id);

    breakpoint_thr_func();

    bin_printf("%ld waiting on term lock\n", lwp_id);

    if ( pthread_mutex_lock(thisArg->term_mutex) != 0 ) {
        perror("pthread_mutex_lock");
        return NULL;
    }

    bin_printf("%ld obtained term lock\n", lwp_id);

    if ( pthread_mutex_unlock(thisArg->term_mutex) != 0 ) {
        perror("pthread_mutex_unlock");
        return NULL;
    }

    bin_printf("%ld released term lock\n", lwp_id);

    return NULL;
}

void start_notification()
{
    bin_printf("In start_notification\n");
}

void term_notification()
{
    bin_printf("In term_notification\n");
}

int main(int argc, char **argv) {
    init_bin();

    if( 2 != argc ) {
        printf("Usage: %s <num. of threads>\n", argv[0]);
        return EXIT_FAILURE;
    }

    signal(SIGUSR1, usr1_handle);

    bin_printf("%d\n", getpid());

    int numThreads;
    sscanf(argv[1], "%d", &numThreads);
    if( numThreads < 1 ) numThreads = 1;

    pthread_t *threads = (pthread_t *)malloc(sizeof(pthread_t)*numThreads);

    pthread_mutex_t *mutex = (pthread_mutex_t *)malloc(sizeof(pthread_mutex_t));

    pthread_mutex_t *term_mutex = (pthread_mutex_t *)malloc(sizeof(pthread_mutex_t));

    if( pthread_mutex_init(mutex, NULL) != 0 ) {
        perror("pthread_mutex_init");
        return EXIT_FAILURE;
    }

    if ( pthread_mutex_init(term_mutex, NULL) != 0 ) {
        perror("pthread_mutex_init");
        return EXIT_FAILURE;
    }

    if( pthread_mutex_lock(mutex) != 0 ) {
        perror("pthread_mutex_lock");
        return EXIT_FAILURE;
    }

    if( pthread_mutex_lock(term_mutex) != 0 ) {
        perror("pthread_mutex_lock");
        return EXIT_FAILURE;
    }

    int i;
    for(i = 0; i < numThreads; ++i) {
        struct threadArg *arg = (struct threadArg *)malloc(sizeof(struct threadArg));
        arg->id = i;
        arg->mutex = mutex;
        arg->term_mutex = term_mutex;

        pthread_create(&threads[i], NULL, &entry, (void *)arg);
    }

    start_notification();

    bin_printf("Received start notification\n");

    if( pthread_mutex_unlock(mutex) != 0 ) {
        perror("pthread_mutex_unlock");
        return EXIT_FAILURE;
    }

    bin_printf("Unlocked mutex, waiting for breakpoint notification\n");

    term_notification();

    bin_printf("Received term notification\n");

    if ( pthread_mutex_unlock(term_mutex) != 0 ) {
        perror("pthread_mutex_unlock");
        return EXIT_FAILURE;
    }

    bin_printf("Unlocked term mutex, joining\n");

    for(i = 0; i < numThreads; ++i ) {
        pthread_join(threads[i], NULL);
        bin_printf("Joined thread %d\n", i);
    }

    return EXIT_SUCCESS;
}
