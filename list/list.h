#ifndef LIST_H
#define LIST_H
typedef struct node {
    int data;
    struct node* next;
    struct node* prev;
} node;

typedef struct list {
    node* head;
} List;

void insert(struct list* l, int value, int at);
int del(struct list* l, int at);
int search(struct list* l, int value);
#endif
