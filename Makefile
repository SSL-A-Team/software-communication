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

# Detect Wireshark personal plugins directory (Linux/macOS).
_WS_PLUGIN_DIR ?= $(shell \
	wireshark -G folders 2>/dev/null \
	| awk -F'\t' '$$1 == "Personal Lua Plugins" {print $$2}' \
)

.PHONY: install-wireshark-plugin
install-wireshark-plugin:
	@if [ -z "$(_WS_PLUGIN_DIR)" ]; then \
		echo "Could not detect Wireshark plugin directory."; \
		echo "Copy wireshark/ateam_radio.lua manually to your personal Lua plugins folder."; \
		echo "(Help → About Wireshark → Folders → Personal Lua Plugins)"; \
		exit 1; \
	fi
	mkdir -p "$(_WS_PLUGIN_DIR)"
	cp wireshark/ateam_radio.lua "$(_WS_PLUGIN_DIR)/"
	@echo "Installed to $(_WS_PLUGIN_DIR)/ateam_radio.lua"
	@echo "Reload in Wireshark with Ctrl+Shift+L or restart."

.PHONY: uninstall-wireshark-plugin
uninstall-wireshark-plugin:
	@if [ -z "$(_WS_PLUGIN_DIR)" ]; then \
		echo "Could not detect Wireshark plugin directory."; \
		echo "Remove wireshark/ateam_radio.lua from your personal Lua plugins folder manually."; \
		exit 1; \
	fi
	rm -f "$(_WS_PLUGIN_DIR)/ateam_radio.lua"
	@echo "Removed $(_WS_PLUGIN_DIR)/ateam_radio.lua"
