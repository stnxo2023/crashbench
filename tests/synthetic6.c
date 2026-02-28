//
#include <stdio.h>

void log_message(const char* user_prefix, const char* msg) {
    char buffer[512];
    snprintf(buffer, sizeof(buffer), "%s: %s\n", user_prefix, msg);
    printf(buffer);
}
