
.PHONY: all
all: sl

.PHONY: sl
sl:
	cargo build

.PHONY: run
run:
	cargo run

.PHONY: clean
clean:
	cargo clean

.PHONY: fmt
fmt:
	cargo fmt

.PHONY: test
test:
	cargo test

.PHONY: distclean
distclean: clean
