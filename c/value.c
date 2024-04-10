#include "value.h"
#include "tree.h"
#include <stdio.h>
#include <stdlib.h>

struct Value *value_int(int val) {
  struct Value *item = malloc(sizeof(struct Value));
  *item = (struct Value){
      .tag = Int,
      .val = val,
  };
  return item;
}

struct Value *value_tree_node(struct TreeNode *node) {
  struct Value *item = malloc(sizeof(struct Value));
  *item = (struct Value){
      .tag = TreeNode,
      .tree_node = node,
  };
  return item;
}

void value_print(struct Value *d) {
  if (d == NULL) {
    return;
  }
  switch (d->tag) {
  case Int:
    printf("item: %d\n", d->val);
    break;
  case TreeNode:
    printf("item: %d\n", d->tree_node->val);
    break;
  default:
    printf("item: None\n");
    break;
  }
}
