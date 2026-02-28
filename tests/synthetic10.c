//
#include <string.h>
#include <stdio.h>

void print_user(const char* long_username) {
    char local_user[16];
    strncpy(local_user, long_username, 16);
    printf("Processing user: %s\n", local_user);
}
