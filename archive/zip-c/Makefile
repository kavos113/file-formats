SRCS=$(wildcard src/*.c)
OBJS=$(SRCS:.c=.o)
TESTS=$(wildcard test/*.c)
TESTOBJS=$(TESTS:.c=.o)

CC=gcc
CFLAGS=-std=c11 -Isrc

.PHONY: all clean test zip

all: zip test

zip: $(OBJS)
	$(CC) $(CFLAGS) -o $@ $^

test: $(TESTOBJS) $(OBJS)
	$(CC) $(CFLAGS) -o $@ $^

%.o: %.c
	$(CC) $(CFLAGS) -c -o $@ $<

clean:
	rm -f $(OBJS) $(TESTOBJS) test zip