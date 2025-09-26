.PHONY: check test test-emu cov clean install-deps

# Default target
all: check test

# Format and lint checks
check:
	cargo fmt --all -- --check
	cargo clippy --all-targets -- -D warnings

# Run tests on host architecture
test:
	cargo test

# Run tests using nextest if available, fallback to cargo test
test-nextest:
	cargo nextest run --profile ci || cargo test

# Run cross-compiled and emulated tests
test-emu:
	TARGET=armv7-unknown-linux-gnueabihf ./scripts/test-emu.sh

# Run coverage analysis
cov:
	RUSTFLAGS="-Zinstrument-coverage" LLVM_PROFILE_FILE="cov-%p-%m.profraw" \
	  cargo test
	grcov . -s . -t lcov --binary-path ./target/debug/ -o lcov.info --ignore "/*" --ignore "target/*"
	@echo "Coverage report generated in lcov.info"
	@if command -v genhtml > /dev/null 2>&1; then \
		genhtml -o coverage lcov.info && echo "HTML coverage report generated in coverage/"; \
	else \
		echo "Install lcov (genhtml) to generate HTML coverage report"; \
	fi

# Clean build artifacts
clean:
	cargo clean
	rm -f cov-*.profraw lcov.info
	rm -rf coverage/

# Install development dependencies
install-deps:
	@echo "Installing development dependencies..."
	cargo install cargo-nextest || echo "cargo-nextest installation failed, will use cargo test"
	rustup component add llvm-tools-preview
	cargo install grcov || echo "grcov installation failed, coverage won't work"

# Build for ARM targets
build-arm:
	cargo build --target armv7-unknown-linux-gnueabihf --release
	cargo build --target aarch64-unknown-linux-gnu --release

# Run emulated binary with healthcheck
run-emu:
	TARGET=armv7-unknown-linux-gnueabihf MODE=release ./scripts/run-emu.sh

# Update snapshot tests
update-snapshots:
	INSTA_ACCEPT=auto cargo test

# Run property-based tests with more cases
prop-test:
	PROPTEST_CASES=10000 cargo test props

# Full CI-like test suite
ci: check test build-arm run-emu cov