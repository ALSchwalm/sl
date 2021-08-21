VERSION=6.0.0

BIN_PATH=target/release/sl
DEB_PATH=sl-$(VERSION)-1.deb

DOCKER_IMG=adamschwalm/sl-build:2

.PHONY: all
all: sl

.PHONY: sl
sl: $(BIN_PATH)

$(BIN_PATH):
	cargo build --release

.PHONY: run
run: $(BIN_PATH)
	cargo run

.PHONY: clean
clean:
	cargo clean
	rm -rf $(DEB_PATH)

.PHONY: fmt
fmt:
	cargo fmt

.PHONY: check-fmt
check-fmt:
	cargo fmt --all -- --check

.PHONY: test
test:
	cargo test

.PHONY: pkg
pkg: $(DEB_PATH)

$(DEB_PATH): $(BIN_PATH)
	rm -rf $(DEB_PATH)
	fpm \
		-s dir -t deb \
		-p $(DEB_PATH) \
		--name sl \
		--license MIT \
		--version $(VERSION) \
		--architecture x86_64 \
		--description "SL" \
		--maintainer "Adam Schwalm <adamschwalm@gmail.com>" \
		--url "https://github.com/mtoyoda/sl" \
		$(BIN_PATH)=/usr/bin/sl sl.1=/usr/share/man/man1/sl.1

.PHONY: distclean
distclean: clean

.PHONY: docker-shell
docker-shell:
	docker run -it --rm -v $(CURDIR):/src $(DOCKER_IMG) /bin/bash

docker-%:
	docker run -it --rm -v $(CURDIR):/src $(DOCKER_IMG) $(MAKE) $*
