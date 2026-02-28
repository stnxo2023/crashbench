//
#include <stdbool.h>

int verify_access(bool is_admin, bool has_token) {
    int access_level;
    if (is_admin) {
        access_level = 5;
    } else if (has_token) {
        access_level = 1;
    }
    if (access_level > 0) {
        return 1;
    }
    return 0;
}
