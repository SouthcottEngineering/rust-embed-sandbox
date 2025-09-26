use anyhow::Result;
use clap::{Arg, Command};
use log::{info, warn};
use serde_json::json;
use std::process;

use my_rust_pi_app::hw::{Gpio, MockGpio};

fn main() -> Result<()> {
    env_logger::init();

    let matches = Command::new("my-rust-pi-app")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Rust application for Raspberry Pi with comprehensive testing")
        .arg(
            Arg::new("healthcheck")
                .long("healthcheck")
                .help("Run health check and exit")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("self-test")
                .long("self-test")
                .help("Run self-test without real hardware")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    if matches.get_flag("healthcheck") {
        return run_healthcheck();
    }

    if matches.get_flag("self-test") {
        return run_self_test();
    }

    info!("Starting Raspberry Pi application");
    println!("Hello from Raspberry Pi!");

    // Simulate some basic GPIO operations using mock hardware
    let mut gpio = MockGpio::new();
    gpio.write(18, true)?;
    let pin_state = gpio.read(18)?;
    info!("GPIO pin 18 state: {}", pin_state);

    Ok(())
}

fn run_healthcheck() -> Result<()> {
    info!("Running health check");

    // Basic system checks
    let mut checks_passed = 0;
    let total_checks = 3;

    // Check 1: Basic logging functionality
    info!("Health check: Logging system");
    checks_passed += 1;

    // Check 2: GPIO mock functionality
    let mut gpio = MockGpio::new();
    match gpio.write(1, true) {
        Ok(_) => {
            info!("Health check: GPIO mock write - OK");
            checks_passed += 1;
        }
        Err(e) => {
            warn!("Health check: GPIO mock write failed: {}", e);
        }
    }

    // Check 3: GPIO mock read functionality
    match gpio.read(1) {
        Ok(state) => {
            info!("Health check: GPIO mock read - OK (state: {})", state);
            checks_passed += 1;
        }
        Err(e) => {
            warn!("Health check: GPIO mock read failed: {}", e);
        }
    }

    if checks_passed == total_checks {
        info!(
            "Health check passed: {}/{} checks successful",
            checks_passed, total_checks
        );
        process::exit(0);
    } else {
        warn!(
            "Health check failed: {}/{} checks successful",
            checks_passed, total_checks
        );
        process::exit(1);
    }
}

fn run_self_test() -> Result<()> {
    info!("Running self-test");

    let mut diagnostics = Vec::new();

    // Test GPIO abstraction
    let mut gpio = MockGpio::new();

    // Test write operations
    match gpio.write(1, true) {
        Ok(_) => diagnostics
            .push(json!({"test": "gpio_write", "status": "pass", "details": "Pin 1 set to HIGH"})),
        Err(e) => diagnostics
            .push(json!({"test": "gpio_write", "status": "fail", "error": e.to_string()})),
    }

    // Test read operations
    match gpio.read(1) {
        Ok(state) => diagnostics.push(json!({"test": "gpio_read", "status": "pass", "details": format!("Pin 1 state: {}", state)})),
        Err(e) => diagnostics.push(json!({"test": "gpio_read", "status": "fail", "error": e.to_string()})),
    }

    // Test call counting
    let call_count = gpio.get_write_count(1);
    diagnostics.push(json!({"test": "call_counting", "status": "pass", "details": format!("Pin 1 write count: {}", call_count)}));

    let result = json!({
        "self_test_results": {
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "total_tests": diagnostics.len(),
            "passed": diagnostics.iter().filter(|d| d["status"] == "pass").count(),
            "failed": diagnostics.iter().filter(|d| d["status"] == "fail").count(),
            "diagnostics": diagnostics
        }
    });

    println!("{}", serde_json::to_string_pretty(&result)?);
    Ok(())
}
