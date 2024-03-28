#include <stdio.h>
#include <stdlib.h>

struct Deque {
  int *front;
  size_t front_start;
  size_t front_end;
  size_t front_cap;

  int *back;
  size_t back_start;
  size_t back_end;
  size_t back_cap;
};

struct Deque *deque_init() {
  int *deque_front = malloc(sizeof(int) * 2);
  int *deque_back = malloc(sizeof(int) * 2);

  struct Deque *d = malloc(sizeof(struct Deque));
  d->front = deque_front;
  d->back = deque_back;

  d->front_start = 0;
  d->front_end = 0;
  d->front_cap = 2;

  d->back_start = 0;
  d->back_end = 0;
  d->back_cap = 2;

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
      printf("Index: %zu, item: %d\n", index, d->front[i]);
      index++;
    }
    // handle the start index. This is later on in the for loop cause if changed
    // to >= d->front_start, it loops back to USIZE_MAX
    printf("Index: %zu, item: %d\n", index, d->front[d->front_start]);
    index++;
  }

  if (d->back_end > d->back_start) {
    for (int i = d->back_start; i < d->back_end; i++) {
      printf("Index: %zu, item: %d\n", index, d->back[i]);
      index++;
    }
  }
}

size_t deque_len(struct Deque *d) {
  return d->front_end - d->front_start + d->back_end - d->back_start;
}

int *array_resize(int *arr, size_t start, size_t end, size_t cap) {
  int *ret = malloc(sizeof(int) * cap * 2);

  int j = 0;
  for (int i = start; i < end; i++) {
    ret[j] = arr[i];
    j++;
  }

  arr = NULL;

  return ret;
}

void deque_push_back(struct Deque *d, int item) {
  if (d->back_end >= d->back_cap) {
    d->back = array_resize(d->back, d->back_start, d->back_end, d->back_cap);
    d->back_cap *= 2;
  }
  d->back[d->back_end] = item;
  d->back_end++;
}

int deque_pop_back(struct Deque *d) {
  if (d->back_end == d->back_start && d->front_end == d->front_start) {
    return -1;
  } else if (d->back_end > d->back_start) {
    int val = d->back[d->back_end - 1];
    d->back_end--;
    return val;
  } else {
    int val = d->front[d->front_start];
    d->front_start++;
    return val;
  }
}

void deque_push_front(struct Deque *d, int item) {
  if (d->front_end >= d->front_cap) {
    d->front =
        array_resize(d->front, d->front_start, d->front_end, d->front_cap);
    d->front_cap *= 2;
  }
  d->front[d->front_end] = item;
  d->front_end++;
}

int deque_pop_front(struct Deque *d) {
  if (d->back_end == d->back_start && d->front_end == d->front_start) {
    return -1;
  } else if (d->front_end > d->front_start) {
    int val = d->front[d->front_end - 1];
    d->front_end--;
    return val;
  } else {
    int val = d->back[d->back_start];
    d->back_start++;
    return val;
  }
}

int main() {
  struct Deque *d = deque_init();
  deque_push_back(d, 4);
  deque_push_back(d, 5);
  deque_push_back(d, 6);
  deque_push_front(d, 3);
  deque_push_front(d, 2);
  deque_push_front(d, 1);
  deque_print(d);
  int val1 = deque_pop_back(d);
  int val2 = deque_pop_back(d);
  int val3 = deque_pop_back(d);
  int val4 = deque_pop_back(d);
  int val5 = deque_pop_front(d);
  int val6 = deque_pop_front(d);
  printf("[%d, %d, %d, %d, %d, %d]\n", val1, val2, val3, val4, val6, val5);
  deque_free(d);
}
