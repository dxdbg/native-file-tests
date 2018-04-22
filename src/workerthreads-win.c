
#include <stdlib.h>
#include <stdio.h>

#define WIN32_LEAN_AND_MEAN
#include "windows.h"

#include "bin_lib.h"

struct threadArg {
    int id;
    HANDLE mutex;
    HANDLE term_mutex;
};

void breakpoint_thr_func() {
    bin_printf("In breakpoint_thr_func\n");
}

DWORD entry(void *arg) {
    DWORD result = ERROR_SUCCESS;

    struct threadArg *thisArg = (struct threadArg *)arg;

    long thread_id = udi_get_thread_id(GetCurrentProcessId());
    bin_printf("%ld waiting on lock\n", thread_id);

    result = WaitForSingleObject(thisArg->mutex, INFINITE);
    if ( result != WAIT_OBJECT_0 ) {
        fprintf(stderr, "WaitForSingleObject: %d\n", GetLastError());
        return GetLastError();
    }

    bin_printf("%ld obtained lock\n", thread_id);

    if ( !ReleaseMutex(thisArg->mutex) ) {
        fprintf(stderr, "ReleaseMutex: %d\n", GetLastError());
        return GetLastError();
    }

    bin_printf("%ld released lock\n", thread_id);

    breakpoint_thr_func();

    bin_printf("%ld waiting on term lock\n", thread_id);

    result = WaitForSingleObject(thisArg->term_mutex, INFINITE);
    if ( result != WAIT_OBJECT_0 ) {
        fprintf(stderr, "WaitForSingleObject: %d\n", GetLastError());
        return GetLastError();
    }

    bin_printf("%ld obtained term lock\n", thread_id);

    if ( !ReleaseMutex(thisArg->term_mutex) ) {
        fprintf(stderr, "ReleaseMutex: %d\n", GetLastError());
        return GetLastError();
    }

    bin_printf("%ld released term lock\n", thread_id);

    return ERROR_SUCCESS;
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
    DWORD result;
    int i, numThreads;
    HANDLE mutex, term_mutex;
    HANDLE *threads;

    init_bin();

    if( 2 != argc ) {
        printf("Usage: %s <num. of threads>\n", argv[0]);
        return EXIT_FAILURE;
    }

    bin_printf("%d\n", GetCurrentProcessId());

    sscanf(argv[1], "%d", &numThreads);
    if( numThreads < 1 ) {
        numThreads = 1;
    }

    mutex = CreateMutex(NULL, TRUE, NULL);
    if ( mutex == NULL ) {
        fprintf(stderr, "CreateMutex: %d\n", GetLastError());
        return EXIT_FAILURE;
    }

    term_mutex = CreateMutex(NULL, TRUE, NULL);
    if ( term_mutex == NULL ) {
        fprintf(stderr, "CreateMutex: %d\n", GetLastError());
        return EXIT_FAILURE;
    }

    threads = (HANDLE *)malloc(sizeof(HANDLE) * numThreads);
    if ( threads == NULL ) {
        fprintf(stderr, "failed to malloc threads structure\n");
        return EXIT_FAILURE;
    }

    for(i = 0; i < numThreads; ++i) {
        struct threadArg *arg = (struct threadArg *)malloc(sizeof(struct threadArg));
        arg->id = i;
        arg->mutex = mutex;
        arg->term_mutex = term_mutex;

        threads[i] = CreateThread(NULL,
                                  0,
                                  (LPTHREAD_START_ROUTINE) entry,
                                  arg,
                                  0,
                                  NULL);
    }

    start_notification();

    bin_printf("Received start notification\n");

    if ( !ReleaseMutex(mutex) ) {
        fprintf(stderr, "ReleaseMutex: %d\n", GetLastError());
        return EXIT_FAILURE;
    }

    bin_printf("Unlocked mutex, waiting for breakpoint notification\n");

    term_notification();

    bin_printf("Received term notification\n");

    if ( !ReleaseMutex(term_mutex) ) {
        fprintf(stderr, "ReleaseMutex: %d\n", GetLastError());
        return EXIT_FAILURE;
    }

    bin_printf("Unlocked term mutex, joining\n");

    WaitForMultipleObjects(numThreads, threads, TRUE, INFINITE);

    for (i = 0; i < numThreads; ++i) {
        CloseHandle(threads[i]);
    }

    CloseHandle(mutex);
    CloseHandle(term_mutex);

    return EXIT_SUCCESS;
}
