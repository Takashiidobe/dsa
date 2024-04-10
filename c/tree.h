#ifndef TREE_H
#define TREE_H

#include <stdbool.h>

struct TreeNode {
  int val;
  struct TreeNode *left;
  struct TreeNode *right;
};

struct TreeNode *tree_new(int val);
bool tree_free(struct TreeNode *root);

#endif
