#include "queue.h"
#include "value.h"
#include <stdio.h>
#include <stdlib.h>

int const static capacity = 2;

struct Deque *deque_init() {
  struct Value **deque_front = malloc(sizeof(struct Value) * 2);
  struct Value **deque_back = malloc(sizeof(struct Value) * 2);

  struct Deque *d = malloc(sizeof(struct Deque));
  *d = (struct Deque){
      .front = deque_front,
      .back = deque_back,
      .front_cap = capacity,
      .back_cap = capacity,
  };

  return d;
}

void deque_free(struct Deque *d) {
  free(d->back);
  free(d->front);
  free(d);
}

void deque_print(struct Deque *d) {
  size_t index = 0;
  if (d->front_end > d->front_start) {
    for (int i = d->front_end - 1; i > d->front_start; i--) {
      printf("Index: %zu ", index);
      value_print(d->front[i]);
      index++;
    }
    // handle the start index. This is later on in the for loop cause if changed
    // to >= d->front_start, it loops back to USIZE_MAX
    printf("Index: %zu ", index);
    value_print(d->front[d->front_start]);
    index++;
  }

  if (d->back_end > d->back_start) {
    for (int i = d->back_start; i < d->back_end; i++) {
      printf("Index: %zu ", index);
      value_print(d->back[i]);
      index++;
    }
  }
}

size_t deque_len(struct Deque *d) {
  return d->front_end - d->front_start + d->back_end - d->back_start;
}

struct Value **array_resize(struct Value **arr, size_t start, size_t end,
                            size_t cap) {
  struct Value **ret = malloc(sizeof(struct Value) * cap * 2);

  printf("resizing array with capacity %zu\n", cap);
  for (int i = start; i < end; i++) {
    printf("%d\n", arr[i]->tree_node->val);
  }

  size_t j = 0;
  for (size_t i = start; i < end; i++) {
    ret[j] = arr[i];
    j++;
  }

  printf("printing resized array with capacity %zu\n", cap);
  for (int i = 0; i < end - start; i++) {
    printf("%d\n", arr[i]->tree_node->val);
  }

  free(arr);
  arr = NULL;

  return ret;
}

void deque_push_back(struct Deque *d, struct Value *item) {
  if (d->back_end >= d->back_cap) {
    d->back = array_resize(d->back, d->back_start, d->back_end, d->back_cap);
    d->back_cap *= 2;
    d->back_start = 0;
    d->back_end = d->back_end - d->back_start;
  }
  d->back[d->back_end] = item;
  d->back_end++;
}

struct Value *deque_pop_back(struct Deque *d) {
  if (d->back_end == d->back_start && d->front_end == d->front_start) {
    struct Value *ret = malloc(sizeof(struct Value));
    *ret = (struct Value){.tag = None};
    return ret;
  } else if (d->back_end > d->back_start) {
    struct Value *val = d->back[d->back_end];
    d->back_end--;
    return val;
  } else {
    struct Value *val = d->front[d->front_start];
    d->front_start++;
    return val;
  }
}

void deque_push_front(struct Deque *d, struct Value *item) {
  if (d->front_end >= d->front_cap) {
    d->front =
        array_resize(d->front, d->front_start, d->front_end, d->front_cap);
    d->front_cap *= 2;
    d->front_start = 0;
    d->front_end = d->front_end - d->front_start;
  }
  d->front[d->front_end] = item;
  d->front_end++;
}

struct Value *deque_pop_front(struct Deque *d) {
  if (d->back_end == d->back_start && d->front_end == d->front_start) {
    struct Value *ret = malloc(sizeof(struct Value));
    *ret = (struct Value){.tag = None};
    return ret;
  } else if (d->front_end > d->front_start) {
    struct Value *val = d->front[d->front_end];
    d->front_end--;
    return val;
  } else {
    struct Value *val = d->back[d->back_start];
    d->back_start++;
    return val;
  }
}
