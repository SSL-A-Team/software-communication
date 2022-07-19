.PHONY: all

.DEFAULT_GOAL := all

bindgen-packets:
	cd ateam-common-packets/rust-lib && \
	cargo build

bindgen-packets-test:
	cd ateam-common-packets/rust-lib && \
	cargo clean && \
	cargo test
