.RECIPEPREFIX = >

# Variables
PROJECT_NAME = rusty-forecast
RELEASE_BINARY = target/release/$(PROJECT_NAME)
INSTALL_DIR = /usr/bin/

# Default target
all: build

# Build target
build:
> cargo build --release

# Install target
install:
> cp $(RELEASE_BINARY) $(INSTALL_DIR)

# Uninstall target
uninstall:
> rm -f $(INSTALL_DIR)/$(PROJECT_NAME)

# Clean target
clean:
> cargo clean

.PHONY: all build install uninstall clean
