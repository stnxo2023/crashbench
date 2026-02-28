//
#include <string.h>

#define MAX_SIZE 256
char global_buffer[MAX_SIZE];

int update_buffer(const char* data, int len) {
    if (len > MAX_SIZE) {
        return -1;
    }
    memcpy(global_buffer, data, len);
    return 0;
}
