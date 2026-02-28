//
#include <stdlib.h>

void* resize_buffer(void* ptr, size_t new_size) {
    void* new_ptr = realloc(ptr, new_size);
    if (new_ptr == NULL) {
        free(ptr);
        return NULL;
    }
    return new_ptr;
}
