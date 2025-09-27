use assert_cmd::prelude::*;
use insta::assert_snapshot;
use std::process::Command;

#[test]
fn help_snapshot() {
    let output = Command::cargo_bin("my-rust-pi-app")
        .unwrap()
        .arg("--help")
        .output()
        .unwrap();

    assert!(output.status.success());
    let text = String::from_utf8_lossy(&output.stdout);
    assert_snapshot!("help_text", text);
}

#[test]
fn version_snapshot() {
    let output = Command::cargo_bin("my-rust-pi-app")
        .unwrap()
        .arg("--version")
        .output()
        .unwrap();

    assert!(output.status.success());
    let text = String::from_utf8_lossy(&output.stdout);
    assert_snapshot!("version_text", text);
}

#[test]
fn self_test_json_structure() {
    let output = Command::cargo_bin("my-rust-pi-app")
        .unwrap()
        .arg("--self-test")
        .output()
        .unwrap();

    assert!(output.status.success());
    let text = String::from_utf8_lossy(&output.stdout);

    // Parse JSON to verify structure but don't snapshot timestamp
    let json: serde_json::Value = serde_json::from_str(&text).expect("Valid JSON");
    assert!(json["self_test_results"].is_object());
    assert!(json["self_test_results"]["total_tests"].is_number());
    assert!(json["self_test_results"]["passed"].is_number());
    assert!(json["self_test_results"]["failed"].is_number());
    assert!(json["self_test_results"]["diagnostics"].is_array());

    // Snapshot without timestamp for stability
    let mut json_without_timestamp = json.clone();
    json_without_timestamp["self_test_results"]
        .as_object_mut()
        .unwrap()
        .remove("timestamp");

    assert_snapshot!(
        "self_test_structure",
        serde_json::to_string_pretty(&json_without_timestamp).unwrap()
    );
}
