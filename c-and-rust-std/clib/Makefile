CFLAGS += -g -O0
OBJS = lib.o

all: libclib.a

libclib.a: $(OBJS)
	$(AR) -crs $@ $^

clean:
	$(RM) *.o *.so *.a
