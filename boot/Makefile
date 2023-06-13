GAS := as
GAS_FLAG := --32

SRCS := $(wildcard *.S)
OBJS := $(patsubst %.S, %.o, $(SRCS))

.PHONY: build clean

build: $(OBJS)

%.o: %.S
	$(GAS) $(GAS_FLAG) -o $@ $<

clean:
	rm -rf $(OBJS)