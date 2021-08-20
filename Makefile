
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

.PHONY: distclean
distclean: clean
