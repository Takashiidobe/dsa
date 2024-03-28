#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

enum EitherSide {
  Left,
  Right,
  Neither,
};

struct Either {
  enum EitherSide side;
  union {
    int left;
    int right;
  };
};

struct Result {
  bool ok;
  int val;
};

int unwrap(struct Result *r) {
  if (r->ok) {
    return r->val;
  } else {
    return NULL;
  }
}

struct Result get_left(struct Either *e) {
  struct Result res = {.ok = false, .val = 0};
  if (e->side == Neither) {
    return res;
  }

  res.ok = true;

  if (e->side == Left) {
    res.val = e->left;
  } else {
    res.val = e->right;
  }

  return res;
}

struct Pair {
  int head;
  int tail;
};

int get_head(struct Pair *p) { return p->head; }
int get_tail(struct Pair *p) { return p->tail; }

struct Vec {
  int *ptr;
  size_t len;
  size_t capacity;
};

struct Vec *vec_init() {
  struct Vec *v = malloc(sizeof(struct Vec));
  v->capacity = 2;
  v->ptr = malloc(sizeof(int) * v->capacity);
  v->len = 0;
  return v;
}

bool vec_free(struct Vec *v) {
  free(v->ptr);
  free(v);
  return true;
}

bool vec_push(struct Vec *v, int item) {
  if (v->len >= v->capacity) {
    int *new_vec = malloc(sizeof(int) * v->capacity * 2);
    for (int i = 0; i < v->len; i++) {
      new_vec[i] = v->ptr[i];
    }
    free(v->ptr);
    v->ptr = new_vec;
    v->capacity *= 2;
  }
  v->ptr[v->len] = item;
  v->len++;
  return true;
}

struct Result vec_pop(struct Vec *v) {
  struct Result r = {.ok = false, .val = 0};
  if (v->len > 0) {
    r.ok = true;
    r.val = v->ptr[v->len - 1];
    v->len--;
  }
  return r;
}

void vec_print(struct Vec *v) {
  for (int i = 0; i < v->len; i++) {
    printf("%d", v->ptr[i]);
    if (i != v->len - 1) {
      printf(" ");
    }
  }
  printf("\n");
}

void vec_debug(struct Vec *v) {
  printf("vec: %p, length: %zu, capacity: %zu\n", v, v->len, v->capacity);
  if (v->len > 0) {
    printf("items: ");
    vec_print(v);
  }
}

int main() {
  struct Vec *v = vec_init();
  vec_push(v, 1);
  vec_push(v, 2);
  vec_push(v, 3);
  vec_push(v, 4);
  vec_push(v, 5);
  vec_push(v, 6);
  vec_push(v, 7);
  vec_push(v, 8);
  vec_push(v, 9);
  vec_debug(v);
  size_t vec_len = v->len;
  for (int i = 0; i < vec_len + 1; i++) {
    struct Result r = vec_pop(v);
    printf("%d\n", r.val);
  }
  vec_free(v);
}
