#ifndef BINARY_SEARCH_TREE_H
#define BINARY_SEARCH_TREE_H

typedef struct BTreeNode {
    int value;
    struct BTreeNode* parent;
    struct BTreeNode* left_child;
    struct BTreeNode* right_child;
} BTreeNode;

BTreeNode* search(BTreeNode* n, int k);
void insert(BTreeNode* bst, BTreeNode* n);
#endif
