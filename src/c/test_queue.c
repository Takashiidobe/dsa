#include "queue.h"
#include "value.h"

int main() {
  struct Deque *d = deque_init();
  struct Value *one = value_int(1);
  struct Value *two = value_int(2);
  struct Value *three = value_int(3);
  struct Value *four = value_int(4);
  struct Value *five = value_int(5);
  struct Value *six = value_int(7);
  deque_push_back(d, one);
  deque_pop_back(d);
  deque_push_back(d, two);
  deque_push_back(d, three);
  deque_print(d);

  deque_free(d);
}
