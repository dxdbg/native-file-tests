#define _GNU_SOURCE 1

#include <unistd.h>
#include <sys/syscall.h>

#include "bin_lib.h"

uint64_t udi_get_thread_id(int pid) {
    return (uint64_t)syscall(SYS_gettid);
}
