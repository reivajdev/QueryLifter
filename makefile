# Nombre del ejecutable final
.PHONY: build release run clean check fmt lint test run-init clean-init
BINARY=querylifter

# --------------------------------
# TAREAS PRINCIPALES
# --------------------------------

build:
	cargo build

release:
	cargo build --release

run:
	cargo run -- init --name test

clean:
	cargo clean

check:
	cargo check

fmt:
	cargo fmt

lint:
	cargo clippy

test:
	cargo test

# --------------------------------
# UTILIDADES PERSONALIZADAS
# --------------------------------

run-init:
	cargo run -- init --name ./test_project

# Borrar carpeta test_project generada por init
clean-init:
	rm -rf ./test_project
