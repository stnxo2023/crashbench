//
#include <string.h>

char* format_status(int status_code) {
    char status_string[64];
    if (status_code == 0) strcpy(status_string, "OK");
    else strcpy(status_string, "ERROR");
    return status_string;
}
