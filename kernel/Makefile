# Basic Info
CUSTOM_TARGET := i386-yamaos

# Directories
SRC_DIR := src
TARGET_DIR := target
DEBUG_BUILD_DIR :=  $(TARGET_DIR)/$(CUSTOM_TARGET)/debug/

# Files
SRCS := $(SRC_DIR)/lib.rs $(SRC_DIR)/drivers/*.rs



.PHONY: build clean

build: $(SRCS)
	cargo build

clean:
	rm -rf $(TARGET_DIR)