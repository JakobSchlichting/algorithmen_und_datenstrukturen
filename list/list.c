#include "list.h"

#include <assert.h>
#include <stdlib.h>
#include <stddef.h>

node* createNode(int value) {
    node* new_node = (node*)malloc(sizeof(node));
    new_node->data = value;
    new_node->prev = NULL;
    new_node->next = NULL;
    return new_node;
}

void insertNode(node* to, int value, int at) {
    assert(to != NULL);
    if(at <= 0) {
        node* new_node = createNode(value);
        new_node->next = to;
        new_node->prev = to->prev;
        to->prev = new_node;
        new_node->prev->next = new_node;
    }
    insertNode(to->next, value, at - 1);
}

// Möglicher mem leak bei at = 0
int deleteNode(node* from, int at) {
    assert(from != NULL);
    if(at <= 0) {
        int value = from->data;
        if(from->prev != NULL) {
            from->prev->next = from->next;
        }
        if(from->next != NULL) {
            from->next->prev = from->prev;
        }
        free(from);
        return value;
    }
    return deleteNode(from->next, at - 1);
}

list new()
