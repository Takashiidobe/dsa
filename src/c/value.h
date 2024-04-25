#ifndef VALUE_H
#define VALUE_H

enum ValueTag {
  Int,
  TreeNode,
  None,
};

struct Value {
  enum ValueTag tag;
  union {
    int val;
    struct TreeNode *tree_node;
  };
};

void value_print(struct Value *d);
struct Value *value_int(int val);
struct Value *value_tree_node(struct TreeNode *node);

#endif
