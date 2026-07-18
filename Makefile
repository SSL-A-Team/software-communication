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

proto-plugin-test:
	cd ateam-common-packets && \
	python3 -m pytest cmake/tests/test_plugin.py -v
test:: proto-plugin-test

proto-cpp-plugin-test:
	cd ateam-common-packets && \
	python3 -m pytest cmake/tests/test_cpp_plugin.py -v
test:: proto-cpp-plugin-test

ssl-proto-test:
	python3 -m pytest ssl-league-protobufs/tests/test_ssl_protos.py -v
test:: ssl-proto-test
