#include "tree.h"
#include "queue.h"
#include "value.h"
#include <stdio.h>
#include <stdlib.h>

struct TreeNode *tree_new(int val) {
  struct TreeNode *t = malloc(sizeof(struct TreeNode));
  t->val = val;
  return t;
}

void tree_print(struct TreeNode *root) {
  if (root == NULL) {
    return;
  }
  tree_print(root->left);
  printf("val: %d\n", root->val);
  tree_print(root->right);
}

bool tree_free(struct TreeNode *root) {
  if (root == NULL) {
    return true;
  }
  if (root->left == NULL && root->right == NULL) {
    free(root);
  }
  return tree_free(root->left) && tree_free(root->right);
}

void bfs(struct TreeNode *root) {
  struct Deque *d = deque_init();
  struct Value *root_item = value_tree_node(root);
  deque_push_back(d, root_item);
  while (deque_len(d)) {
    struct Value *curr = deque_pop_front(d);
    struct TreeNode *left_node = curr->tree_node->left;
    struct TreeNode *right_node = curr->tree_node->right;
    if (left_node) {
      struct Value *left = value_tree_node(left_node);
      deque_push_back(d, left);
    }
    if (right_node) {
      struct Value *right = value_tree_node(right_node);
      deque_push_back(d, right);
    }
    deque_print(d);
  }
}

int main() {
  struct TreeNode *root = tree_new(10);
  root->left = tree_new(5);
  root->right = tree_new(15);
  root->left->left = tree_new(3);
  root->left->right = tree_new(7);
  root->right->left = tree_new(12);
  root->right->right = tree_new(18);
  bfs(root);
  tree_free(root);
}
