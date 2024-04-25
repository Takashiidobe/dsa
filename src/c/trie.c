#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

struct TrieNode {
  char c;
  bool at_end;
  struct TrieNode **children;
};

void trie_print(struct TrieNode *t, size_t depth) {
  printf("Depth: %zu, Char: %c\n", depth, t->c);
  if (t->at_end) {
    return;
  }
  for (int i = 0; i < 26; i++) {
    if (t->children[i] != NULL) {
      trie_print(t->children[i], depth + 1);
    }
  }
}

bool trie_free(struct TrieNode *t) {
  if (t != NULL) {
    for (int i = 0; i < 26; i++) {
      if (t->children[i] != NULL) {
        trie_free(t->children[i]);
      }
    }
    free(t->children);
    free(t);
  }
  return true;
}

struct TrieNode *trie_new(char c) {
  struct TrieNode *curr = malloc(sizeof(struct TrieNode));
  curr->c = c;
  curr->children = malloc(sizeof(struct TrieNode) * 26);

  return curr;
}

struct TrieNode *trie_init(const char *str, size_t len) {
  struct TrieNode *curr = trie_new('$');
  struct TrieNode *head = curr;
  struct TrieNode *ret = NULL;

  for (int i = 0; i < len; i++) {
    char c = str[i];
    struct TrieNode *next = trie_new(c);
    curr->children['z' - c] = next;
    curr = next;
    if (i == 0) {
      ret = curr;
    }
  }

  free(head);
  curr->at_end = true;

  return ret;
}

void trie_add(struct TrieNode *t, const char *str, size_t len) {
  struct TrieNode *dummy = trie_new('$');
  struct TrieNode *curr = dummy;
  curr->children['z' - t->c] = t;

  for (int i = 0; i < len; i++) {
    char c = str[i];
    if (curr->children['z' - c] == NULL) {
      struct TrieNode *next = trie_new(c);
      curr->children['z' - c] = next;
    }
    curr = curr->children['z' - c];
  }
  free(dummy);
  curr->at_end = true;
}

bool trie_has(struct TrieNode *t, const char *str, size_t len) {
  struct TrieNode *dummy = trie_new('$');
  struct TrieNode *curr = dummy;
  curr->children['z' - t->c] = t;

  for (int i = 0; i < len; i++) {
    char c = str[i];
    if (curr->children['z' - c] == NULL) {
      return false;
    }
    curr = curr->children['z' - c];
  }
  return curr->at_end;
}

int main() {
  struct TrieNode *t = trie_init("hello", 5);
  trie_add(t, "help", 4);
  trie_add(t, "helde", 5);
  trie_print(t, 0);

  printf("%d\n", trie_has(t, "hell", 4));
  printf("%d\n", trie_has(t, "help", 4));
  printf("%d\n", trie_has(t, "held", 4));

  trie_free(t);
}
