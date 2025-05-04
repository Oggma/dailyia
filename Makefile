VUE_DIR = ./dailyia-ui
RUST_DIR = ./dailyia-api
VUE_OUTPUT_DIR = ../../build-ui/dist
RUST_OUTPUT_DIR = ../../build-api/target/release

.PHONY: all vue rust clean

all: vue rust

vue:
	@echo "Building Vue.js application..."
	mkdir -p $(VUE_OUTPUT_DIR)
	cd $(VUE_DIR) && npm install && npm run build -- --output-path=$(VUE_OUTPUT_DIR)
	@echo "Running Vue.js application..."
	cd $(VUE_DIR) && npm run serve

rust:
	@echo "Building Rust application..."
	mkdir -p $(RUST_OUTPUT_DIR)
	cd $(RUST_DIR) && cargo build --release --target-dir=$(RUST_OUTPUT_DIR)
	@echo "Running Rust application..."
	cd $(RUST_OUTPUT_DIR) && ./dailyia-api

clean:
	@echo "Cleaning Vue.js build..."
	rm -rf $(VUE_OUTPUT_DIR)
	@echo "Cleaning Rust build..."
	rm -rf $(RUST_OUTPUT_DIR)