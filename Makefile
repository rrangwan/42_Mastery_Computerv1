# Variables
CARGO := cargo
RELEASE := /target/release/computor1

# Commands
CARGO = cargo

# Targets
all: release

build:
	$(CARGO) build

release:
	$(CARGO) build --release

clean:
	$(CARGO) clean

fclean: clean
	rm -rf target

re: fclean all

.PHONY: all build release clean fclean re
