use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use footprint_core::JsonValue;

static UNIQUIFIER: AtomicU64 = AtomicU64::new(0);

fn unique_store_path() -> PathBuf {
    let mut path = env::temp_dir();
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    let seq = UNIQUIFIER.fetch_add(1, Ordering::Relaxed);
    path.push(format!(
        "footprint-core-test-{}-{now}-{seq}.jsonl",
        std::process::id()
    ));
    path
}

fn run_cli(store_path: &Path, input: &str) -> Output {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_footprint-core"));
    cmd.arg("--store")
        .arg(store_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child = cmd.spawn().expect("spawn footprint-core");
    child
        .stdin
        .take()
        .expect("stdin is piped")
        .write_all(input.as_bytes())
        .expect("write stdin");
    child.wait_with_output().expect("collect output")
}

fn json_object(value: &JsonValue) -> &std::collections::BTreeMap<String, JsonValue> {
    value.as_object().expect("JSON object")
}

fn json_string<'a>(obj: &'a std::collections::BTreeMap<String, JsonValue>, key: &str) -> &'a str {
    obj.get(key)
        .and_then(JsonValue::as_string)
        .expect("JSON string field")
}

#[test]
fn cli_appends_canonical_event_jsonl_and_reports_success() {
    let store_path = unique_store_path();
    if store_path.exists() {
        fs::remove_file(&store_path).expect("remove leftover file");
    }

    let input = r#"{
      "traceId": "trace-001",
      "deviceId": "device-a",
      "capturedAt": "2026-02-01T21:34:56.789+09:00",
      "kind": "manual",
      "payload": {"zeta": 1, "alpha": {"beta": 2, "aardvark": 1}}
    }"#;

    let output = run_cli(&store_path, input);
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8(output.stdout).expect("stdout utf8");
    let stdout_value = footprint_core::parse_json(&stdout).expect("stdout JSON");
    let stdout_obj = json_object(&stdout_value);
    assert_eq!(
        stdout_obj.get("ok").and_then(JsonValue::as_bool),
        Some(true)
    );
    assert_eq!(json_string(stdout_obj, "event_id"), "trace-001");
    assert_eq!(
        json_string(stdout_obj, "committed_at"),
        "2026-02-01T12:34:56.789Z"
    );
    assert_eq!(
        json_string(stdout_obj, "store_path"),
        store_path.display().to_string()
    );

    let committed_at = json_string(stdout_obj, "committed_at");
    assert!(committed_at.ends_with('Z'));
    let millis = committed_at
        .split('.')
        .nth(1)
        .expect("fractional part")
        .strip_suffix('Z')
        .expect("Z suffix");
    assert_eq!(millis.len(), 3);
    assert!(millis.chars().all(|c| c.is_ascii_digit()));

    let file_content = fs::read_to_string(&store_path).expect("read store");
    let lines: Vec<&str> = file_content.lines().collect();
    assert_eq!(lines.len(), 1);

    let row = footprint_core::parse_json(lines[0]).expect("store row JSON");
    let row_obj = json_object(&row);
    assert_eq!(json_string(row_obj, "event_id"), "trace-001");
    assert_eq!(
        json_string(row_obj, "committed_at"),
        "2026-02-01T12:34:56.789Z"
    );
    assert_eq!(json_string(row_obj, "blank"), "NONE");
    assert_eq!(json_string(row_obj, "time_source"), "MANUAL");
    assert_eq!(json_string(row_obj, "kind"), "manual");
    assert_eq!(json_string(row_obj, "device_id"), "device-a");

    let payload = row_obj
        .get("payload")
        .and_then(JsonValue::as_object)
        .expect("payload");
    let alpha = payload
        .get("alpha")
        .and_then(JsonValue::as_object)
        .expect("payload.alpha");
    assert_eq!(
        alpha
            .get("aardvark")
            .and_then(JsonValue::as_u64)
            .expect("payload.alpha.aardvark"),
        1
    );
    assert_eq!(
        alpha
            .get("beta")
            .and_then(JsonValue::as_u64)
            .expect("payload.alpha.beta"),
        2
    );
    assert_eq!(
        payload
            .get("zeta")
            .and_then(JsonValue::as_u64)
            .expect("payload.zeta"),
        1
    );

    let expected_bytes = file_content.len() as u64;
    assert_eq!(
        stdout_obj
            .get("bytes_appended")
            .and_then(JsonValue::as_u64)
            .expect("bytes_appended"),
        expected_bytes
    );

    fs::remove_file(&store_path).expect("cleanup store file");
}

#[test]
fn cli_rejects_unknown_fields_and_returns_json_error() {
    let store_path = unique_store_path();
    if store_path.exists() {
        fs::remove_file(&store_path).expect("remove leftover file");
    }

    let input = r#"{
      "traceId": "trace-002",
      "deviceId": "device-b",
      "capturedAt": "2026-02-01T12:34:56.789Z",
      "kind": "sensor",
      "unknown": "not-allowed"
    }"#;

    let output = run_cli(&store_path, input);
    assert!(
        !output.status.success(),
        "unexpected success; stdout: {} stderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8(output.stdout).expect("stdout utf8");
    let stdout_value = footprint_core::parse_json(&stdout).expect("stdout JSON");
    let stdout_obj = json_object(&stdout_value);
    assert_eq!(
        stdout_obj.get("ok").and_then(JsonValue::as_bool),
        Some(false)
    );

    let error_obj = stdout_obj
        .get("error")
        .and_then(JsonValue::as_object)
        .expect("error object");
    assert_eq!(json_string(error_obj, "code"), "invalid_input");

    assert!(
        !store_path.exists(),
        "store file should not be created on invalid input"
    );
}
