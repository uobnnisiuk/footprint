use std::collections::BTreeMap;
use std::env;
use std::ffi::OsString;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

pub const STORE_ENV_VAR: &str = "FOOTPRINT_CORE_STORE_PATH";

#[derive(Debug, Clone)]
pub struct CoreError {
    pub code: &'static str,
    pub message: String,
}

impl CoreError {
    fn new(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }

    fn invalid_args(message: impl Into<String>) -> Self {
        Self::new("invalid_args", message)
    }

    fn invalid_input(message: impl Into<String>) -> Self {
        Self::new("invalid_input", message)
    }

    fn io(message: impl Into<String>) -> Self {
        Self::new("io_error", message)
    }
}

impl std::fmt::Display for CoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for CoreError {}

#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Number(String),
    String(String),
    Array(Vec<JsonValue>),
    Object(BTreeMap<String, JsonValue>),
}

impl JsonValue {
    pub fn as_object(&self) -> Option<&BTreeMap<String, JsonValue>> {
        if let Self::Object(map) = self {
            Some(map)
        } else {
            None
        }
    }

    pub fn as_string(&self) -> Option<&str> {
        if let Self::String(value) = self {
            Some(value)
        } else {
            None
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        if let Self::Bool(value) = self {
            Some(*value)
        } else {
            None
        }
    }

    pub fn as_u64(&self) -> Option<u64> {
        match self {
            Self::Number(raw) => raw.parse::<u64>().ok(),
            _ => None,
        }
    }

    pub fn to_json_string(&self) -> String {
        let mut out = String::new();
        write_json_value(self, &mut out);
        out
    }
}

#[derive(Debug, Clone)]
pub struct SuccessOutput {
    pub ok: bool,
    pub event_id: String,
    pub committed_at: String,
    pub store_path: String,
    pub bytes_appended: u64,
}

#[derive(Debug, Clone, Copy)]
enum TraceKind {
    Beacon,
    Manual,
    Sensor,
    Network,
}

impl TraceKind {
    fn from_str(value: &str) -> Option<Self> {
        match value {
            "beacon" => Some(Self::Beacon),
            "manual" => Some(Self::Manual),
            "sensor" => Some(Self::Sensor),
            "network" => Some(Self::Network),
            _ => None,
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::Beacon => "beacon",
            Self::Manual => "manual",
            Self::Sensor => "sensor",
            Self::Network => "network",
        }
    }

    fn time_source(self) -> &'static str {
        match self {
            Self::Manual => "MANUAL",
            _ => "SYSTEM",
        }
    }
}

#[derive(Debug, Clone)]
struct TraceInput {
    trace_id: String,
    device_id: String,
    captured_at: String,
    kind: TraceKind,
    payload: JsonValue,
}

pub fn run<I>(args: I, stdin_json: &str) -> Result<SuccessOutput, CoreError>
where
    I: IntoIterator<Item = OsString>,
{
    let store_override = parse_store_override(args)?;
    let store_path = resolve_store_path(store_override)?;
    let trace = parse_trace(stdin_json)?;
    append_trace(trace, &store_path)
}

pub fn parse_json(input: &str) -> Result<JsonValue, CoreError> {
    let mut parser = JsonParser::new(input);
    let value = parser.parse_value()?;
    parser.skip_whitespace();
    if !parser.is_eof() {
        return Err(CoreError::invalid_input(
            "invalid JSON: trailing characters after root value",
        ));
    }
    Ok(value)
}

pub fn render_success_json(output: &SuccessOutput) -> String {
    let mut map = BTreeMap::new();
    map.insert(
        String::from("bytes_appended"),
        JsonValue::Number(output.bytes_appended.to_string()),
    );
    map.insert(
        String::from("committed_at"),
        JsonValue::String(output.committed_at.clone()),
    );
    map.insert(
        String::from("event_id"),
        JsonValue::String(output.event_id.clone()),
    );
    map.insert(String::from("ok"), JsonValue::Bool(output.ok));
    map.insert(
        String::from("store_path"),
        JsonValue::String(output.store_path.clone()),
    );
    JsonValue::Object(map).to_json_string()
}

pub fn render_error_json(error: &CoreError) -> String {
    let mut details = BTreeMap::new();
    details.insert(
        String::from("code"),
        JsonValue::String(String::from(error.code)),
    );
    details.insert(
        String::from("message"),
        JsonValue::String(error.message.clone()),
    );

    let mut root = BTreeMap::new();
    root.insert(String::from("error"), JsonValue::Object(details));
    root.insert(String::from("ok"), JsonValue::Bool(false));
    JsonValue::Object(root).to_json_string()
}

fn parse_store_override<I>(args: I) -> Result<Option<PathBuf>, CoreError>
where
    I: IntoIterator<Item = OsString>,
{
    let mut iter = args.into_iter();
    let _program = iter.next();

    let mut store_path: Option<PathBuf> = None;
    while let Some(arg) = iter.next() {
        if arg == "--store" {
            let Some(path) = iter.next() else {
                return Err(CoreError::invalid_args("--store requires a path argument"));
            };
            store_path = Some(PathBuf::from(path));
            continue;
        }

        return Err(CoreError::invalid_args(format!(
            "unsupported argument: {}",
            arg.to_string_lossy()
        )));
    }

    Ok(store_path)
}

fn resolve_store_path(override_path: Option<PathBuf>) -> Result<PathBuf, CoreError> {
    if let Some(path) = override_path {
        return Ok(path);
    }

    if let Ok(path) = env::var(STORE_ENV_VAR) {
        if !path.trim().is_empty() {
            return Ok(PathBuf::from(path));
        }
    }

    if let Ok(state_home) = env::var("XDG_STATE_HOME") {
        if !state_home.trim().is_empty() {
            return Ok(PathBuf::from(state_home).join("footprint/core/events.jsonl"));
        }
    }

    let home = env::var("HOME")
        .map_err(|_| CoreError::io("HOME is not set; cannot resolve default store path"))?;
    Ok(PathBuf::from(home).join(".local/state/footprint/core/events.jsonl"))
}

fn parse_trace(raw: &str) -> Result<TraceInput, CoreError> {
    if raw.trim().is_empty() {
        return Err(CoreError::invalid_input("stdin is empty"));
    }

    let value = parse_json(raw)?;
    let JsonValue::Object(mut map) = value else {
        return Err(CoreError::invalid_input(
            "input must be a JSON object that matches trace schema",
        ));
    };

    let trace_id = take_required_string(&mut map, "traceId")?;
    if trace_id.is_empty() {
        return Err(CoreError::invalid_input(
            "traceId must be a non-empty string",
        ));
    }

    let device_id = take_required_string(&mut map, "deviceId")?;
    if device_id.is_empty() {
        return Err(CoreError::invalid_input(
            "deviceId must be a non-empty string",
        ));
    }

    let captured_at = take_required_string(&mut map, "capturedAt")?;
    let kind_raw = take_required_string(&mut map, "kind")?;
    let kind = TraceKind::from_str(&kind_raw).ok_or_else(|| {
        CoreError::invalid_input("kind must be one of: beacon, manual, sensor, network")
    })?;

    let payload = map.remove("payload").unwrap_or(JsonValue::Null);
    if !matches!(payload, JsonValue::Object(_) | JsonValue::Null) {
        return Err(CoreError::invalid_input(
            "payload must be an object when present",
        ));
    }

    if let Some((unknown, _)) = map.iter().next() {
        return Err(CoreError::invalid_input(format!(
            "unknown field in input: {}",
            unknown
        )));
    }

    Ok(TraceInput {
        trace_id,
        device_id,
        captured_at,
        kind,
        payload,
    })
}

fn take_required_string(
    map: &mut BTreeMap<String, JsonValue>,
    key: &str,
) -> Result<String, CoreError> {
    let value = map
        .remove(key)
        .ok_or_else(|| CoreError::invalid_input(format!("missing required field: {}", key)))?;
    match value {
        JsonValue::String(s) => Ok(s),
        _ => Err(CoreError::invalid_input(format!(
            "field {} must be a string",
            key
        ))),
    }
}

fn append_trace(trace: TraceInput, store_path: &Path) -> Result<SuccessOutput, CoreError> {
    let normalized_time = normalize_timestamp(&trace.captured_at)?;

    let mut event = BTreeMap::new();
    event.insert(
        String::from("blank"),
        JsonValue::String(String::from("NONE")),
    );
    event.insert(
        String::from("committed_at"),
        JsonValue::String(normalized_time.clone()),
    );
    event.insert(
        String::from("device_id"),
        JsonValue::String(trace.device_id.clone()),
    );
    event.insert(
        String::from("event_id"),
        JsonValue::String(trace.trace_id.clone()),
    );
    event.insert(
        String::from("kind"),
        JsonValue::String(String::from(trace.kind.as_str())),
    );
    event.insert(String::from("payload"), trace.payload);
    event.insert(
        String::from("time_source"),
        JsonValue::String(String::from(trace.kind.time_source())),
    );

    let line = format!("{}\n", JsonValue::Object(event).to_json_string());

    if let Some(parent) = store_path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent).map_err(|e| {
                CoreError::io(format!("create store directory {}: {e}", parent.display()))
            })?;
        }
    }

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(store_path)
        .map_err(|e| CoreError::io(format!("open store {}: {e}", store_path.display())))?;

    file.write_all(line.as_bytes())
        .map_err(|e| CoreError::io(format!("write store {}: {e}", store_path.display())))?;
    file.flush()
        .map_err(|e| CoreError::io(format!("flush store {}: {e}", store_path.display())))?;
    file.sync_data()
        .map_err(|e| CoreError::io(format!("sync store {}: {e}", store_path.display())))?;

    Ok(SuccessOutput {
        ok: true,
        event_id: trace.trace_id,
        committed_at: normalized_time,
        store_path: store_path.display().to_string(),
        bytes_appended: line.len() as u64,
    })
}

fn normalize_timestamp(value: &str) -> Result<String, CoreError> {
    let epoch_millis = parse_rfc3339_to_epoch_millis(value)?;
    Ok(format_epoch_millis_utc(epoch_millis))
}

fn parse_rfc3339_to_epoch_millis(input: &str) -> Result<i64, CoreError> {
    let bytes = input.as_bytes();
    let mut i = 0usize;

    let year = parse_fixed_digits(bytes, &mut i, 4)? as i32;
    expect_char(bytes, &mut i, '-')?;
    let month = parse_fixed_digits(bytes, &mut i, 2)? as u32;
    expect_char(bytes, &mut i, '-')?;
    let day = parse_fixed_digits(bytes, &mut i, 2)? as u32;
    expect_char(bytes, &mut i, 'T')?;
    let hour = parse_fixed_digits(bytes, &mut i, 2)? as u32;
    expect_char(bytes, &mut i, ':')?;
    let minute = parse_fixed_digits(bytes, &mut i, 2)? as u32;
    expect_char(bytes, &mut i, ':')?;
    let second = parse_fixed_digits(bytes, &mut i, 2)? as u32;

    if month == 0 || month > 12 {
        return Err(CoreError::invalid_input("capturedAt has invalid month"));
    }
    let max_day = days_in_month(year, month);
    if day == 0 || day > max_day {
        return Err(CoreError::invalid_input("capturedAt has invalid day"));
    }
    if hour > 23 {
        return Err(CoreError::invalid_input("capturedAt has invalid hour"));
    }
    if minute > 59 {
        return Err(CoreError::invalid_input("capturedAt has invalid minute"));
    }
    if second > 59 {
        return Err(CoreError::invalid_input("capturedAt has invalid second"));
    }

    let mut millis = 0i64;
    if i < bytes.len() && bytes[i] == b'.' {
        i += 1;
        let frac_start = i;
        while i < bytes.len() && bytes[i].is_ascii_digit() {
            i += 1;
        }
        if i == frac_start {
            return Err(CoreError::invalid_input(
                "capturedAt has invalid fractional seconds",
            ));
        }
        let frac = &input[frac_start..i];
        millis = fractional_to_millis(frac);
    }

    if i >= bytes.len() {
        return Err(CoreError::invalid_input(
            "capturedAt is missing timezone designator",
        ));
    }

    let offset_minutes = match bytes[i] {
        b'Z' => {
            i += 1;
            0i64
        }
        b'+' | b'-' => {
            let sign = if bytes[i] == b'+' { 1i64 } else { -1i64 };
            i += 1;
            let off_hour = parse_fixed_digits(bytes, &mut i, 2)? as i64;
            expect_char(bytes, &mut i, ':')?;
            let off_min = parse_fixed_digits(bytes, &mut i, 2)? as i64;
            if off_hour > 23 || off_min > 59 {
                return Err(CoreError::invalid_input(
                    "capturedAt has invalid timezone offset",
                ));
            }
            sign * (off_hour * 60 + off_min)
        }
        _ => {
            return Err(CoreError::invalid_input(
                "capturedAt has invalid timezone designator",
            ))
        }
    };

    if i != bytes.len() {
        return Err(CoreError::invalid_input(
            "capturedAt has trailing characters",
        ));
    }

    let days = days_from_civil(year, month, day);
    let day_seconds = (hour as i64) * 3600 + (minute as i64) * 60 + (second as i64);
    let utc_seconds = days
        .checked_mul(86_400)
        .and_then(|v| v.checked_add(day_seconds))
        .and_then(|v| v.checked_sub(offset_minutes * 60))
        .ok_or_else(|| CoreError::invalid_input("capturedAt out of supported range"))?;

    utc_seconds
        .checked_mul(1_000)
        .and_then(|v| v.checked_add(millis))
        .ok_or_else(|| CoreError::invalid_input("capturedAt out of supported range"))
}

fn parse_fixed_digits(bytes: &[u8], i: &mut usize, width: usize) -> Result<u32, CoreError> {
    if *i + width > bytes.len() {
        return Err(CoreError::invalid_input(
            "capturedAt does not match RFC3339 format",
        ));
    }
    let mut value = 0u32;
    for _ in 0..width {
        let ch = bytes[*i];
        if !ch.is_ascii_digit() {
            return Err(CoreError::invalid_input(
                "capturedAt does not match RFC3339 format",
            ));
        }
        value = value * 10 + u32::from(ch - b'0');
        *i += 1;
    }
    Ok(value)
}

fn expect_char(bytes: &[u8], i: &mut usize, expected: char) -> Result<(), CoreError> {
    if *i >= bytes.len() || bytes[*i] != expected as u8 {
        return Err(CoreError::invalid_input(
            "capturedAt does not match RFC3339 format",
        ));
    }
    *i += 1;
    Ok(())
}

fn fractional_to_millis(frac: &str) -> i64 {
    let mut digits = [b'0'; 3];
    let bytes = frac.as_bytes();
    for (idx, slot) in digits.iter_mut().enumerate() {
        if idx < bytes.len() {
            *slot = bytes[idx];
        }
    }
    let hundreds = i64::from(digits[0] - b'0') * 100;
    let tens = i64::from(digits[1] - b'0') * 10;
    let ones = i64::from(digits[2] - b'0');
    hundreds + tens + ones
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => 0,
    }
}

fn days_from_civil(year: i32, month: u32, day: u32) -> i64 {
    let mut y = i64::from(year);
    let m = i64::from(month);
    let d = i64::from(day);
    y -= if m <= 2 { 1 } else { 0 };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let doy = (153 * (m + if m > 2 { -3 } else { 9 }) + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

fn civil_from_days(days: i64) -> (i32, u32, u32) {
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = mp + if mp < 10 { 3 } else { -9 };
    let year = y + if month <= 2 { 1 } else { 0 };
    (year as i32, month as u32, day as u32)
}

fn div_floor_i128(a: i128, b: i128) -> i128 {
    let mut q = a / b;
    let r = a % b;
    if r != 0 && ((r > 0) != (b > 0)) {
        q -= 1;
    }
    q
}

fn mod_floor_i128(a: i128, b: i128) -> i128 {
    a - div_floor_i128(a, b) * b
}

fn format_epoch_millis_utc(epoch_millis: i64) -> String {
    let ms = i128::from(epoch_millis);
    let total_seconds = div_floor_i128(ms, 1_000);
    let millis = mod_floor_i128(ms, 1_000) as i64;
    let days = div_floor_i128(total_seconds, 86_400);
    let sec_of_day = mod_floor_i128(total_seconds, 86_400) as i64;

    let hour = sec_of_day / 3_600;
    let minute = (sec_of_day % 3_600) / 60;
    let second = sec_of_day % 60;
    let (year, month, day) = civil_from_days(days as i64);

    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:03}Z",
        year, month, day, hour, minute, second, millis
    )
}

fn write_json_value(value: &JsonValue, out: &mut String) {
    match value {
        JsonValue::Null => out.push_str("null"),
        JsonValue::Bool(v) => {
            if *v {
                out.push_str("true")
            } else {
                out.push_str("false")
            }
        }
        JsonValue::Number(raw) => out.push_str(raw),
        JsonValue::String(value) => write_json_string(value, out),
        JsonValue::Array(items) => {
            out.push('[');
            for (idx, item) in items.iter().enumerate() {
                if idx > 0 {
                    out.push(',');
                }
                write_json_value(item, out);
            }
            out.push(']');
        }
        JsonValue::Object(map) => {
            out.push('{');
            let mut first = true;
            for (key, item) in map {
                if !first {
                    out.push(',');
                }
                first = false;
                write_json_string(key, out);
                out.push(':');
                write_json_value(item, out);
            }
            out.push('}');
        }
    }
}

fn write_json_string(value: &str, out: &mut String) {
    out.push('"');
    for ch in value.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\u{08}' => out.push_str("\\b"),
            '\u{0C}' => out.push_str("\\f"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if c <= '\u{1F}' => {
                let escaped = format!("\\u{:04X}", c as u32);
                out.push_str(&escaped);
            }
            c => out.push(c),
        }
    }
    out.push('"');
}

struct JsonParser<'a> {
    src: &'a [u8],
    idx: usize,
}

impl<'a> JsonParser<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            src: input.as_bytes(),
            idx: 0,
        }
    }

    fn is_eof(&self) -> bool {
        self.idx >= self.src.len()
    }

    fn peek(&self) -> Option<u8> {
        self.src.get(self.idx).copied()
    }

    fn bump(&mut self) -> Option<u8> {
        let ch = self.peek()?;
        self.idx += 1;
        Some(ch)
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if matches!(ch, b' ' | b'\n' | b'\r' | b'\t') {
                self.idx += 1;
            } else {
                break;
            }
        }
    }

    fn parse_value(&mut self) -> Result<JsonValue, CoreError> {
        self.skip_whitespace();
        let Some(ch) = self.peek() else {
            return Err(CoreError::invalid_input(
                "invalid JSON: unexpected end of input",
            ));
        };

        match ch {
            b'{' => self.parse_object(),
            b'[' => self.parse_array(),
            b'"' => self.parse_string().map(JsonValue::String),
            b't' => self.parse_literal("true", JsonValue::Bool(true)),
            b'f' => self.parse_literal("false", JsonValue::Bool(false)),
            b'n' => self.parse_literal("null", JsonValue::Null),
            b'-' | b'0'..=b'9' => self.parse_number().map(JsonValue::Number),
            _ => Err(CoreError::invalid_input(
                "invalid JSON: unexpected token while parsing value",
            )),
        }
    }

    fn parse_literal(&mut self, literal: &str, value: JsonValue) -> Result<JsonValue, CoreError> {
        let bytes = literal.as_bytes();
        if self.src.len().saturating_sub(self.idx) < bytes.len() {
            return Err(CoreError::invalid_input("invalid JSON literal"));
        }
        if &self.src[self.idx..self.idx + bytes.len()] == bytes {
            self.idx += bytes.len();
            Ok(value)
        } else {
            Err(CoreError::invalid_input("invalid JSON literal"))
        }
    }

    fn parse_object(&mut self) -> Result<JsonValue, CoreError> {
        self.expect_byte(b'{')?;
        self.skip_whitespace();

        let mut map = BTreeMap::new();
        if self.peek() == Some(b'}') {
            self.idx += 1;
            return Ok(JsonValue::Object(map));
        }

        loop {
            self.skip_whitespace();
            let key = self.parse_string()?;
            self.skip_whitespace();
            self.expect_byte(b':')?;
            let value = self.parse_value()?;

            if map.insert(key.clone(), value).is_some() {
                return Err(CoreError::invalid_input(format!(
                    "invalid JSON: duplicate key {}",
                    key
                )));
            }

            self.skip_whitespace();
            match self.bump() {
                Some(b',') => continue,
                Some(b'}') => break,
                _ => {
                    return Err(CoreError::invalid_input(
                        "invalid JSON object: expected ',' or '}'",
                    ))
                }
            }
        }

        Ok(JsonValue::Object(map))
    }

    fn parse_array(&mut self) -> Result<JsonValue, CoreError> {
        self.expect_byte(b'[')?;
        self.skip_whitespace();

        let mut items = Vec::new();
        if self.peek() == Some(b']') {
            self.idx += 1;
            return Ok(JsonValue::Array(items));
        }

        loop {
            let value = self.parse_value()?;
            items.push(value);
            self.skip_whitespace();
            match self.bump() {
                Some(b',') => continue,
                Some(b']') => break,
                _ => {
                    return Err(CoreError::invalid_input(
                        "invalid JSON array: expected ',' or ']'",
                    ))
                }
            }
        }

        Ok(JsonValue::Array(items))
    }

    fn parse_string(&mut self) -> Result<String, CoreError> {
        self.expect_byte(b'"')?;
        let mut out = String::new();

        loop {
            let Some(ch) = self.bump() else {
                return Err(CoreError::invalid_input(
                    "invalid JSON string: unexpected end of input",
                ));
            };

            match ch {
                b'"' => break,
                b'\\' => {
                    let Some(escaped) = self.bump() else {
                        return Err(CoreError::invalid_input(
                            "invalid JSON string: incomplete escape sequence",
                        ));
                    };
                    match escaped {
                        b'"' => out.push('"'),
                        b'\\' => out.push('\\'),
                        b'/' => out.push('/'),
                        b'b' => out.push('\u{08}'),
                        b'f' => out.push('\u{0C}'),
                        b'n' => out.push('\n'),
                        b'r' => out.push('\r'),
                        b't' => out.push('\t'),
                        b'u' => {
                            let code = self.parse_hex16()?;
                            if (0xD800..=0xDBFF).contains(&code) {
                                self.expect_byte(b'\\')?;
                                self.expect_byte(b'u')?;
                                let low = self.parse_hex16()?;
                                if !(0xDC00..=0xDFFF).contains(&low) {
                                    return Err(CoreError::invalid_input(
                                        "invalid JSON string: invalid surrogate pair",
                                    ));
                                }
                                let high_ten = u32::from(code - 0xD800);
                                let low_ten = u32::from(low - 0xDC00);
                                let cp = 0x10000 + ((high_ten << 10) | low_ten);
                                let Some(unicode) = char::from_u32(cp) else {
                                    return Err(CoreError::invalid_input(
                                        "invalid JSON string: invalid unicode scalar",
                                    ));
                                };
                                out.push(unicode);
                            } else if (0xDC00..=0xDFFF).contains(&code) {
                                return Err(CoreError::invalid_input(
                                    "invalid JSON string: lone low surrogate",
                                ));
                            } else {
                                let Some(unicode) = char::from_u32(u32::from(code)) else {
                                    return Err(CoreError::invalid_input(
                                        "invalid JSON string: invalid unicode scalar",
                                    ));
                                };
                                out.push(unicode);
                            }
                        }
                        _ => {
                            return Err(CoreError::invalid_input(
                                "invalid JSON string: unsupported escape character",
                            ))
                        }
                    }
                }
                other if other < 0x20 => {
                    return Err(CoreError::invalid_input(
                        "invalid JSON string: unescaped control character",
                    ))
                }
                other => {
                    let Some(ch) = decode_utf8_char(other, self) else {
                        return Err(CoreError::invalid_input(
                            "invalid JSON string: malformed UTF-8",
                        ));
                    };
                    out.push(ch);
                }
            }
        }

        Ok(out)
    }

    fn parse_hex16(&mut self) -> Result<u16, CoreError> {
        if self.idx + 4 > self.src.len() {
            return Err(CoreError::invalid_input(
                "invalid JSON string: incomplete \\u escape",
            ));
        }
        let mut value = 0u16;
        for _ in 0..4 {
            let ch = self.src[self.idx];
            self.idx += 1;
            value <<= 4;
            value |= match ch {
                b'0'..=b'9' => u16::from(ch - b'0'),
                b'a'..=b'f' => u16::from(ch - b'a') + 10,
                b'A'..=b'F' => u16::from(ch - b'A') + 10,
                _ => {
                    return Err(CoreError::invalid_input(
                        "invalid JSON string: bad hex in \\u escape",
                    ))
                }
            };
        }
        Ok(value)
    }

    fn parse_number(&mut self) -> Result<String, CoreError> {
        let start = self.idx;

        if self.peek() == Some(b'-') {
            self.idx += 1;
        }

        match self.peek() {
            Some(b'0') => {
                self.idx += 1;
            }
            Some(b'1'..=b'9') => {
                self.idx += 1;
                while matches!(self.peek(), Some(b'0'..=b'9')) {
                    self.idx += 1;
                }
            }
            _ => {
                return Err(CoreError::invalid_input(
                    "invalid JSON number: missing integer part",
                ))
            }
        }

        if self.peek() == Some(b'.') {
            self.idx += 1;
            let frac_start = self.idx;
            while matches!(self.peek(), Some(b'0'..=b'9')) {
                self.idx += 1;
            }
            if self.idx == frac_start {
                return Err(CoreError::invalid_input(
                    "invalid JSON number: missing fractional digits",
                ));
            }
        }

        if matches!(self.peek(), Some(b'e' | b'E')) {
            self.idx += 1;
            if matches!(self.peek(), Some(b'+' | b'-')) {
                self.idx += 1;
            }
            let exp_start = self.idx;
            while matches!(self.peek(), Some(b'0'..=b'9')) {
                self.idx += 1;
            }
            if self.idx == exp_start {
                return Err(CoreError::invalid_input(
                    "invalid JSON number: missing exponent digits",
                ));
            }
        }

        let raw = std::str::from_utf8(&self.src[start..self.idx])
            .map_err(|_| CoreError::invalid_input("invalid JSON number encoding"))?;
        Ok(normalize_number_lexeme(raw))
    }

    fn expect_byte(&mut self, expected: u8) -> Result<(), CoreError> {
        match self.bump() {
            Some(actual) if actual == expected => Ok(()),
            _ => Err(CoreError::invalid_input(
                "invalid JSON syntax: unexpected token",
            )),
        }
    }
}

fn decode_utf8_char(first: u8, parser: &mut JsonParser<'_>) -> Option<char> {
    if first < 0x80 {
        return Some(char::from(first));
    }

    let width = if first & 0b1110_0000 == 0b1100_0000 {
        2
    } else if first & 0b1111_0000 == 0b1110_0000 {
        3
    } else if first & 0b1111_1000 == 0b1111_0000 {
        4
    } else {
        return None;
    };

    let mut buf = [0u8; 4];
    buf[0] = first;
    for slot in buf.iter_mut().take(width).skip(1) {
        let next = parser.bump()?;
        if next & 0b1100_0000 != 0b1000_0000 {
            return None;
        }
        *slot = next;
    }

    std::str::from_utf8(&buf[..width]).ok()?.chars().next()
}

fn normalize_number_lexeme(raw: &str) -> String {
    let s = raw.replace('E', "e");
    if let Some(idx) = s.find('e') {
        let (mantissa_raw, exponent_raw) = s.split_at(idx);
        let mut mantissa = trim_decimal_trailing_zeros(mantissa_raw);
        let exponent = &exponent_raw[1..];
        let (sign, digits) = if let Some(rest) = exponent.strip_prefix('+') {
            ("", rest)
        } else if let Some(rest) = exponent.strip_prefix('-') {
            ("-", rest)
        } else {
            ("", exponent)
        };
        let trimmed_digits = digits.trim_start_matches('0');
        let exp_digits = if trimmed_digits.is_empty() {
            "0"
        } else {
            trimmed_digits
        };
        if mantissa.is_empty() {
            mantissa.push('0');
        }
        return format!("{mantissa}e{sign}{exp_digits}");
    }

    trim_decimal_trailing_zeros(&s)
}

fn trim_decimal_trailing_zeros(raw: &str) -> String {
    if !raw.contains('.') {
        return raw.to_string();
    }
    let mut out = raw.to_string();
    while out.ends_with('0') {
        out.pop();
    }
    if out.ends_with('.') {
        out.pop();
    }
    if out == "-0" {
        return String::from("0");
    }
    out
}

#[cfg(test)]
mod tests {
    use super::{
        format_epoch_millis_utc, normalize_timestamp, parse_json, parse_rfc3339_to_epoch_millis,
        JsonValue,
    };

    #[test]
    fn normalize_timestamp_to_utc_millis() {
        let got = normalize_timestamp("2026-02-01T21:34:56.789+09:00").unwrap();
        assert_eq!(got, "2026-02-01T12:34:56.789Z");
    }

    #[test]
    fn normalize_timestamp_truncates_sub_millis() {
        let got = normalize_timestamp("2026-02-01T12:34:56.789987Z").unwrap();
        assert_eq!(got, "2026-02-01T12:34:56.789Z");
    }

    #[test]
    fn normalize_timestamp_handles_negative_epoch() {
        let millis = parse_rfc3339_to_epoch_millis("1969-12-31T23:59:59.900Z").unwrap();
        assert_eq!(millis, -100);
        assert_eq!(format_epoch_millis_utc(millis), "1969-12-31T23:59:59.900Z");
    }

    #[test]
    fn parse_json_sorts_object_keys() {
        let value = parse_json(r#"{"b":1,"a":{"y":2,"x":1}}"#).unwrap();
        assert_eq!(value.to_json_string(), r#"{"a":{"x":1,"y":2},"b":1}"#);
    }

    #[test]
    fn parse_json_null_and_bool() {
        let value = parse_json(r#"{"ok":true,"payload":null}"#).unwrap();
        let root = value.as_object().unwrap();
        assert_eq!(root.get("ok"), Some(&JsonValue::Bool(true)));
        assert_eq!(root.get("payload"), Some(&JsonValue::Null));
    }
}
