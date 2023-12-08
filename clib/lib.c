#include <stddef.h>
#include <stdio.h>

int square(int x) { return x * x; }

struct span {
  void *ptr;
  size_t len;
};

struct span_pair {
  struct span fst;
  struct span snd;
};

struct span span_create(void *ptr, size_t len) {
  return (struct span){
      .ptr = ptr,
      .len = len,
  };
}

void show_span_pair(struct span_pair *pair) {
  puts("pair:");
  printf("\tptr: %p\n\tlen: %lu\n", pair->fst.ptr, pair->fst.len);
  printf("\tptr: %p\n\tlen: %lu\n", pair->snd.ptr, pair->snd.len);
}
