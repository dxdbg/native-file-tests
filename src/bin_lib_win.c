
#define WIN32_LEAN_AND_MEAN
#include "windows.h"

#include "bin_lib.h"

uint64_t udi_get_thread_id(int pid) {
    return GetCurrentThreadId();
}
