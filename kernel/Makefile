# Basic Info
CUSTOM_TARGET := i386-yamaos

# Directories
SRC_DIR := src
TARGET_DIR := target
DEBUG_BUILD_DIR :=  $(TARGET_DIR)/$(CUSTOM_TARGET)/debug/

# Files
SRCS := $(SRC_DIR)/lib.rs $(SRC_DIR)/drivers/*.rs



.PHONY: build clean

CARGO_OPTIONS := -Z build-std=core,compiler_builtins -Z build-std-features=compiler-builtins-mem
CARGO_TARGET := --target $(CUSTOM_TARGET).json
CARGO_OPTIONS += $(CARGO_TARGET)

build: $(SRCS)
	cargo build $(CARGO_OPTIONS)

clean:
	rm -rf $(TARGET_DIR)