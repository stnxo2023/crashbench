//
#include <stdlib.h>
#include <string.h>

typedef struct { int id; char data[60]; } Item;

Item* allocate_items(size_t count) {
    size_t total_size = count * sizeof(Item);
    Item* items = (Item*)malloc(total_size);
    if (!items) return NULL;
    for (size_t i = 0; i < count; i++) {
        items[i].id = i;
        memset(items[i].data, 0, 60);
    }
    return items;
}
