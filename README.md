# Rust Pi Development Environment

A comprehensive Rust development environment for Raspberry Pi with emulated hardware and robust testing.

## Features

- **Hardware Abstraction Layer**: GPIO, I2C, and SPI traits with full mock implementations for testing
- **Comprehensive Test Suite**: Unit, integration, property-based, and snapshot tests (22 tests total)
- **ARM Cross-Compilation**: Support for `aarch64-unknown-linux-gnu` (64-bit ARM)
- **QEMU Emulation**: Automated testing under ARM emulation
- **CLI Interface**: Health checks and self-testing capabilities
- **CI/CD Pipeline**: GitHub Actions with coverage reporting and ARM emulation

## Quick Start

### Development Container

Open in VS Code dev container for a pre-configured environment:

```bash
# The devcontainer includes all necessary tools:
# - Rust toolchain with ARM targets
# - QEMU emulation
# - Cross-compilation tools
# - Testing frameworks
```

### Local Development

```bash
# Run checks (format + clippy)
make check

# Run tests
make test

# Build for ARM
make build-arm

# Run emulated healthcheck
make run-emu
```

## CLI Usage

```bash
# Show help
cargo run -- --help

# Run healthcheck
cargo run -- --healthcheck

# Run self-test with JSON output
cargo run -- --self-test

# Default application
cargo run
```

## Testing Philosophy

### Test Types

1. **Unit Tests**: Pure logic, hardware abstraction layer tests
2. **Integration Tests**: CLI interface testing with `assert_cmd`
3. **Property-Based Tests**: Input validation with `proptest`
4. **Snapshot Tests**: Stable output verification with `insta`
5. **Contract Tests**: Hardware abstraction contracts with mock verification

### Hardware Mocking

```rust
use my_rust_pi_app::hw::{Gpio, MockGpio};

// Create mock GPIO with scripted responses
let mut gpio = MockGpio::new();
gpio.set_scripted_responses(1, vec![true, false, true]);

// Test with failure injection
gpio.set_pin_failure(2);
assert!(gpio.write(2, true).is_err());

// Verify call counts
assert_eq!(gpio.get_write_count(1), 2);
```

## ARM Cross-Compilation & Emulation

### Cross-Compilation

```bash
# Build for 64-bit ARM (Raspberry Pi 3/4/5)
cargo build --target aarch64-unknown-linux-gnu --release
cargo build --target aarch64-unknown-linux-gnu --release
```

### Emulated Testing

```bash
# Run emulated ARM binary with healthcheck
TARGET=aarch64-unknown-linux-gnu MODE=release ./scripts/run-emu.sh

# Run tests under emulation
TARGET=aarch64-unknown-linux-gnu ./scripts/test-emu.sh
```

## CI/CD Pipeline

The GitHub Actions workflow:

1. **Format & Lint**: `cargo fmt --check` and `cargo clippy`
2. **Host Tests**: Full test suite on host architecture
3. **Cross-Compilation**: Build for ARM targets
4. **Emulated Healthcheck**: Verify ARM binary runs under QEMU
5. **Coverage**: LLVM source-based coverage with 75% threshold

## Coverage Reporting

```bash
# Generate coverage report
make cov

# View HTML report (if genhtml is installed)
open coverage/index.html
```

## Project Structure

```
├── .cargo/config.toml          # Cross-compilation configuration
├── .devcontainer/              # VS Code dev container setup
├── .github/workflows/ci.yml    # CI/CD pipeline
├── scripts/                    # Emulation and testing scripts
├── src/
│   ├── main.rs                 # CLI application
│   ├── lib.rs                  # Library exports
│   └── hw.rs                   # Hardware abstraction layer
├── tests/                      # Integration tests
│   ├── cli_smoke.rs            # CLI interface tests
│   ├── hardware_contracts.rs   # Hardware abstraction tests
│   ├── props.rs                # Property-based tests
│   ├── snap_cli.rs             # Snapshot tests
│   └── snapshots/              # Snapshot files
└── Makefile                    # Task orchestration
```

## Dependencies

### Runtime Dependencies
- `anyhow`: Error handling
- `clap`: CLI argument parsing
- `env_logger`: Logging
- `serde_json`: JSON serialization
- `chrono`: Timestamp generation

### Development Dependencies
- `assert_cmd`: CLI testing
- `assert_fs`: Filesystem testing
- `predicates`: Test assertions
- `proptest`: Property-based testing
- `insta`: Snapshot testing
- `serial_test`: Test isolation

## License

This project is created as a comprehensive template for Rust Pi development with emulated hardware testing.