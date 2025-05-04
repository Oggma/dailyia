VUE_DIR = ./dailyia-ui
RUST_DIR = ./dailyia-api
VUE_OUTPUT_DIR = ../build-ui
RUST_OUTPUT_DIR = ../../build-api

.PHONY: all vue rust clean

all: vue rust

vue:
	@echo "Building Vue.js application..."
	rm -rf $(VUE_OUTPUT_DIR)
	mkdir -p $(VUE_OUTPUT_DIR)
	npm --prefix $(VUE_DIR) install && npm --prefix $(VUE_DIR) run-script build-only
	mv $(VUE_DIR)/dist $(VUE_OUTPUT_DIR)
	@echo "Running Vue.js application..."
	cd $(VUE_OUTPUT_DIR) && serve -s dist

rust:
	@echo "Building Rust application..."
	cd $(RUST_DIR)
	mkdir -p $(RUST_OUTPUT_DIR)
	cargo build --release --target-dir=$(RUST_OUTPUT_DIR)
	@echo "Running Rust application..."
	cd $(RUST_OUTPUT_DIR)/release && ./dailyia-api

clean:
	@echo "Cleaning Vue.js build..."
	rm -rf $(VUE_OUTPUT_DIR)
	@echo "Cleaning Rust build..."
	rm -rf $(RUST_OUTPUT_DIR)