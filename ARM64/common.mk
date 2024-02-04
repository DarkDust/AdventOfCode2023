DAY = $(shell basename "$(shell pwd)")

%.o: %.s
	as -I../lib -g -o $@ $<

lib_SOURCES := $(wildcard ../lib/*.s)
lib_OBJECTS := $(lib_SOURCES:.s=.o)

$(DAY): $(DAY).o $(lib_OBJECTS)
	ld -o "$@" $^

.PHONY: clean
clean:
	rm -f $(DAY) *.o
