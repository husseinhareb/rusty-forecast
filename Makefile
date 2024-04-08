.RECIPEPREFIX = >

# Variables
PROJECT_NAME = rusty-forecast
DEBUG_BINARY = target/debug/$(PROJECT_NAME)
RELEASE_BINARY = target/release/$(PROJECT_NAME)
INSTALL_DIR = /usr/bin

# Default target
all: build

# Build target
build:
> cargo build

# Install target
install: build
> sudo cp $(DEBUG_BINARY) $(INSTALL_DIR)

# Uninstall target
uninstall:
> sudo rm -f $(INSTALL_DIR)/$(PROJECT_NAME)

# Clean target
clean:
> cargo clean

.PHONY: all build install uninstall clean