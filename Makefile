.PHONY: clean

.PHONY: all
.DEFAULT_GOAL := all

.PHONY: test

bindgen-packets-clean:
	cd ateam-common-packets/rust-lib && \
	rm -f src/stspin_bindings.rs && \
	cargo clean
clean:: bindgen-packets-clean

bindgen-packets:
	cd ateam-common-packets/rust-lib && \
	cargo build
all:: bindgen-packets

bindgen-packets-test: bindgen-packets-clean
	cd ateam-common-packets/rust-lib && \
	cargo test
test:: bindgen-packets-test
