.PHONY: check test test-emu cov cov-grcov clean install-deps

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
	TARGET=aarch64-unknown-linux-gnu ./scripts/test-emu.sh

# Run coverage analysis using cargo-llvm-cov
cov:
	@echo "[cov] Ensuring nightly toolchain is installed..."
	@rustup toolchain install nightly --component llvm-tools-preview 2>/dev/null || true
	@echo "[cov] Ensuring cargo-llvm-cov is installed..."
	@cargo install cargo-llvm-cov --quiet || true
	@echo "[cov] Cleaning previous coverage artifacts..."
	@rm -f cov-*.profraw lcov.info
	@rm -rf coverage/
	@echo "[cov] Running tests with coverage instrumentation..."
	@cargo +nightly llvm-cov --workspace --all-features --fail-under-lines 75 --lcov --output-path lcov.info --ignore-filename-regex '(/.cargo/|/tests/.*/snapshots/)'
	@echo "[cov] Coverage report generated in lcov.info"
	@if command -v genhtml > /dev/null 2>&1; then \
		echo "[cov] Generating HTML coverage report..."; \
		genhtml lcov.info --output-directory coverage --branch-coverage && echo "[cov] HTML coverage report generated in coverage/"; \
	else \
		echo "[cov] Install lcov (genhtml) to generate HTML coverage report"; \
	fi

# Alternative coverage using grcov (legacy approach - commented out by default)
# Uncomment and use 'make cov-grcov' if you prefer the old grcov-based workflow
# cov-grcov:
# 	@echo "[cov-grcov] Using legacy grcov approach (requires nightly)..."
# 	@rustup toolchain install nightly --component llvm-tools-preview 2>/dev/null || true
# 	@cargo install grcov --quiet || true
# 	@rm -f cov-*.profraw lcov.info
# 	@rm -rf coverage/
# 	@RUSTFLAGS="-Zinstrument-coverage" LLVM_PROFILE_FILE="cov-%p-%m.profraw" \
# 	  cargo +nightly test
# 	@grcov . -s . -t lcov --binary-path ./target/debug/ -o lcov.info --ignore "/*" --ignore "target/*"
# 	@echo "[cov-grcov] Coverage report generated in lcov.info"
# 	@if command -v genhtml > /dev/null 2>&1; then \
# 		genhtml -o coverage lcov.info && echo "[cov-grcov] HTML coverage report generated in coverage/"; \
# 	else \
# 		echo "[cov-grcov] Install lcov (genhtml) to generate HTML coverage report"; \
# 	fi

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
	cargo build --target aarch64-unknown-linux-gnu --release

# Run emulated binary with healthcheck
run-emu:
	TARGET=aarch64-unknown-linux-gnu MODE=release ./scripts/run-emu.sh

# Update snapshot tests
update-snapshots:
	INSTA_ACCEPT=auto cargo test

# Run property-based tests with more cases
prop-test:
	PROPTEST_CASES=10000 cargo test props

# Full CI-like test suite
ci: check test build-arm run-emu cov