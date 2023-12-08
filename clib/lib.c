#include <stddef.h>
int square(int x) {
  return x * x;
}

struct span {
  void *ptr;
  size_t len;
};

struct span_pair {
  struct span fst;
  struct span snd;
};

struct span span_create(void *ptr, size_t len) {
  return (struct span) {
    .ptr = ptr,
    .len = len,
  };
}
