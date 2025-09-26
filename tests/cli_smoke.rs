use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn prints_version() {
    let mut cmd = Command::cargo_bin("my-rust-pi-app").unwrap();
    cmd.arg("--version");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("my-rust-pi-app"));
}

#[test]
fn shows_help() {
    let mut cmd = Command::cargo_bin("my-rust-pi-app").unwrap();
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Rust application for Raspberry Pi"));
}

#[test]
fn healthcheck_passes() {
    let mut cmd = Command::cargo_bin("my-rust-pi-app").unwrap();
    cmd.arg("--healthcheck");
    cmd.assert()
        .success()
        .code(0);
}

#[test]
fn self_test_produces_json() {
    let mut cmd = Command::cargo_bin("my-rust-pi-app").unwrap();
    cmd.arg("--self-test");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("self_test_results"))
        .stdout(predicate::str::contains("timestamp"))
        .stdout(predicate::str::contains("diagnostics"));
}

#[test]
fn default_run_succeeds() {
    let mut cmd = Command::cargo_bin("my-rust-pi-app").unwrap();
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Hello from Raspberry Pi!"));
}