.PHONY: all

.DEFAULT_GOAL := all

bindgen-packets:
	cd ateam-common-packets/ && \
	cargo build

bindgen-packets-test:
	cd ateam-common-packets/ && \
	cargo clean && \
	cargo test
