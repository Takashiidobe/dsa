#ifndef QUEUE_H
#define QUEUE_H

#include "tree.h"
#include <stddef.h>

struct Deque {
  struct Value **front;
  size_t front_start;
  size_t front_end;
  size_t front_cap;

  struct Value **back;
  size_t back_start;
  size_t back_end;
  size_t back_cap;
};

struct Deque *deque_init();
void deque_free(struct Deque *d);
void deque_print(struct Deque *d);
size_t deque_len(struct Deque *d);

struct Value **array_resize(struct Value **arr, size_t start, size_t end,
                            size_t cap);
void deque_push_back(struct Deque *d, struct Value *item);
struct Value *deque_pop_back(struct Deque *d);
void deque_push_front(struct Deque *d, struct Value *item);
struct Value *deque_pop_front(struct Deque *d);

#endif
