#define _GNU_SOURCE
#include <fcntl.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>
#include <sys/syscall.h>
#include <unistd.h>

extern void *__libc_malloc(size_t size);
extern void *__libc_calloc(size_t count, size_t size);
extern void *__libc_realloc(void *ptr, size_t size);
extern void __libc_free(void *ptr);

static volatile uint64_t malloc_calls;
static volatile uint64_t calloc_calls;
static volatile uint64_t realloc_calls;
static volatile uint64_t free_calls;
static volatile uint64_t allocated_bytes;
static volatile uint64_t reallocated_bytes;
static volatile int shutting_down;

static void write_u64(char *out, size_t *pos, uint64_t value) {
    char digits[24];
    size_t n = 0;
    do {
        digits[n++] = (char)('0' + (value % 10));
        value /= 10;
    } while (value != 0);
    while (n != 0) out[(*pos)++] = digits[--n];
}

static void write_field(char *out, size_t *pos, const char *name, uint64_t value, int comma) {
    const char *prefix = comma ? ",\"" : "{\"";
    while (*prefix) out[(*pos)++] = *prefix++;
    while (*name) out[(*pos)++] = *name++;
    out[(*pos)++] = '\"';
    out[(*pos)++] = ':';
    write_u64(out, pos, value);
}

static void emit_profile(void) {
    if (__atomic_exchange_n(&shutting_down, 1, __ATOMIC_RELAXED)) return;
    const char *path = getenv("VELQU_ALLOC_PROFILE");
    if (path == NULL || *path == '\0') return;

    char buffer[512];
    size_t pos = 0;
    write_field(buffer, &pos, "mallocCalls", malloc_calls, 0);
    write_field(buffer, &pos, "callocCalls", calloc_calls, 1);
    write_field(buffer, &pos, "reallocCalls", realloc_calls, 1);
    write_field(buffer, &pos, "freeCalls", free_calls, 1);
    write_field(buffer, &pos, "allocatedBytes", allocated_bytes, 1);
    write_field(buffer, &pos, "reallocatedBytes", reallocated_bytes, 1);
    buffer[pos++] = '}';
    buffer[pos++] = '\n';

    int fd = (int)syscall(SYS_openat, AT_FDCWD, path, O_WRONLY | O_CREAT | O_TRUNC, 0644);
    if (fd < 0) return;
    size_t written = 0;
    while (written < pos) {
        long n = syscall(SYS_write, fd, buffer + written, pos - written);
        if (n <= 0) break;
        written += (size_t)n;
    }
    syscall(SYS_close, fd);
}

__attribute__((constructor)) static void alloc_tracer_init(void) {
    malloc_calls = 0;
    calloc_calls = 0;
    realloc_calls = 0;
    free_calls = 0;
    allocated_bytes = 0;
    reallocated_bytes = 0;
    shutting_down = 0;
}

__attribute__((destructor)) static void alloc_tracer_fini(void) {
    emit_profile();
}

void *malloc(size_t size) {
    void *ptr = __libc_malloc(size);
    if (!shutting_down) {
        __atomic_add_fetch(&malloc_calls, 1, __ATOMIC_RELAXED);
        __atomic_add_fetch(&allocated_bytes, size, __ATOMIC_RELAXED);
    }
    return ptr;
}

void *calloc(size_t count, size_t size) {
    void *ptr = __libc_calloc(count, size);
    if (!shutting_down) {
        __atomic_add_fetch(&calloc_calls, 1, __ATOMIC_RELAXED);
        __atomic_add_fetch(&allocated_bytes, (uint64_t)count * size, __ATOMIC_RELAXED);
    }
    return ptr;
}

void *realloc(void *ptr, size_t size) {
    void *out = __libc_realloc(ptr, size);
    if (!shutting_down) {
        __atomic_add_fetch(&realloc_calls, 1, __ATOMIC_RELAXED);
        __atomic_add_fetch(&reallocated_bytes, size, __ATOMIC_RELAXED);
    }
    return out;
}

void free(void *ptr) {
    __libc_free(ptr);
    if (!shutting_down) __atomic_add_fetch(&free_calls, 1, __ATOMIC_RELAXED);
}
