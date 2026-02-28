//
#include <string.h>

void build_path(char* directory, const char* filename) {
    char path[128];
    strncpy(path, directory, sizeof(path) - 1);
    path[sizeof(path) - 1] = '\0';
    strncat(path, "/", sizeof(path) - strlen(path) - 1);
    strncat(path, filename, sizeof(path) - strlen(path));
}
