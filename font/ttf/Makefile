SRCS=$(wildcard src/*.c)
OBJS=$(SRCS:.c=.o)

CC=gcc
CFLAGS=-std=c11

all: readttf

readttf: $(OBJS)
	$(CC) -o $@ $^ $(CFLAGS)

clean:
	rm -f readttf $(OBJS)

.PHONY: all clean