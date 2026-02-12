SRCS=$(wildcard src/*.c)
OBJS=$(SRCS:.c=.o)

CC=gcc
CFLAGS=-std=c11

all: readpe

readpe: $(OBJS)
	$(CC) -o $@ $^ $(CFLAGS)

clean:
	rm -f readpe $(OBJS)

.PHONY: all clean