//
#include <stdlib.h>

struct Node { int val; struct Node* next; };

void free_list(struct Node* head) {
    struct Node* current = head;
    while (current != NULL) {
        free(current);
        current = current->next;
    }
}
