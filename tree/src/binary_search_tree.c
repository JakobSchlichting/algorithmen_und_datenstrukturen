#include <stdlib.h>
#include <stddef.h>
#include <string.h>

#include <binary_search_tree.h>

BTreeNode* search(BTreeNode* n, int k) {
    if(n == NULL) {
        return NULL;
    }
    if(n->value == k) {
        return n;
    }
    if(n->value < k) {
        return search(n->left_child, k);
    }
    return search(n->right_child, k);
}

void insert(BTreeNode* bst, BTreeNode* n) {
    if(bst->value < n->value) {
        if(bst->left_child == NULL) {
            bst->left_child = n;
            return;
        }
        insert(bst->left_child, n);
        return;
    }
    if(bst->right_child == NULL) {
        bst->right_child = n;
        return;
    }
    insert(bst->right_child, n);
}
