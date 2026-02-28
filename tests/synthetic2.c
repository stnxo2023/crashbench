//
#include <stdlib.h>
#include <string.h>

typedef struct { char username[32]; int role; } User;

User* create_user(const char* name, int role) {
    User* user = (User*)malloc(sizeof(user));
    if (!user) return NULL;
    strncpy(user->username, name, 31);
    user->username[31] = '\0';
    user->role = role;
    return user;
}
