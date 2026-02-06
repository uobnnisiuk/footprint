#!/usr/bin/env python3
from __future__ import annotations

import argparse
import csv
import json
import statistics
from datetime import datetime, timezone
from pathlib import Path
from typing import Any, Dict, Iterable, List, Optional, Tuple


def parse_ts(value: Any) -> Optional[datetime]:
    if not isinstance(value, str):
        return None
    text = value.strip()
    if not text:
        return None
    if text.endswith("Z"):
        text = text[:-1] + "+00:00"
    try:
        return datetime.fromisoformat(text)
    except ValueError:
        pass
    for fmt in (
        "%Y-%m-%dT%H:%M:%S.%f%z",
        "%Y-%m-%dT%H:%M:%S%z",
        "%Y-%m-%dT%H:%M:%S.%f",
        "%Y-%m-%dT%H:%M:%S",
    ):
        try:
            return datetime.strptime(text, fmt)
        except ValueError:
            continue
    return None


def coerce_float(value: Any) -> Optional[float]:
    if value is None:
        return None
    if isinstance(value, bool):
        return None
    if isinstance(value, (int, float)):
        return float(value)
    try:
        return float(str(value).strip())
    except (ValueError, TypeError):
        return None


def normalize_str(value: Any) -> str:
    if value is None:
        return "unknown"
    if isinstance(value, bool):
        return "true" if value else "false"
    text = str(value).strip()
    return text if text else "unknown"


def normalize_number(value: Any) -> str:
    num = coerce_float(value)
    if num is None:
        return normalize_str(value)
    if num.is_integer():
        return str(int(num))
    return f"{num:g}"


def normalize_low_power(record: Dict[str, Any]) -> str:
    if "low_power_tag" in record:
        return normalize_str(record.get("low_power_tag"))
    if "low_power" in record:
        low_power = record.get("low_power")
        if isinstance(low_power, bool):
            return "low" if low_power else "normal"
        if isinstance(low_power, (int, float)):
            return "low" if low_power else "normal"
        return normalize_str(low_power)
    return "unknown"


def gather_files(paths: Iterable[str]) -> List[Path]:
    files: List[Path] = []
    for raw in paths:
        path = Path(raw)
        if path.is_dir():
            files.extend(sorted(path.rglob("*.jsonl")))
        else:
            files.append(path)
    return files


def format_ratio(success: int, total: int) -> str:
    if total <= 0:
        return "n/a"
    return f"{success}/{total}={success / total:.2f}"


def write_demo_logs(target_dir: Path) -> List[Path]:
    demo_dir = target_dir / "demo_logs"
    demo_dir.mkdir(parents=True, exist_ok=True)
    ios_log = demo_dir / "demo_ios.jsonl"
    android_log = demo_dir / "demo_android.jsonl"

    ios_events = [
        {
            "ts": "2025-02-06T10:00:00.000Z",
            "trial_id": "demo-ios-1",
            "device_role": "A",
            "os": "iOS",
            "app_state": "background",
            "low_power": True,
            "proximity_tag": "room",
            "distance_m_tag": "1",
            "duration_target_min": "5",
            "peer_id": "peer-ios",
            "event_type": "trial_start",
            "battery_pct": 82,
        },
        {
            "ts": "2025-02-06T10:02:30.000Z",
            "trial_id": "demo-ios-1",
            "device_role": "A",
            "os": "iOS",
            "app_state": "background",
            "low_power": True,
            "proximity_tag": "room",
            "distance_m_tag": "1",
            "duration_target_min": "5",
            "peer_id": "peer-ios",
            "event_type": "first_detect",
            "rssi": -62,
            "battery_pct": 81,
        },
    ]

    android_events = [
        {
            "ts": "2025-02-06T11:00:00Z",
            "trial_id": "demo-android-1",
            "device_role": "alpha",
            "os": "Android",
            "app_state": "fg",
            "low_power_tag": "normal",
            "proximity_tag": "near",
            "distance_m_tag": "2",
            "duration_target_min": 5,
            "peer_id": "peer-android",
            "event_type": "trial_start",
            "battery_pct": 67,
        },
        {
            "ts": "2025-02-06T11:08:00Z",
            "trial_id": "demo-android-1",
            "device_role": "alpha",
            "os": "Android",
            "app_state": "service",
            "low_power_tag": "normal",
            "proximity_tag": "near",
            "distance_m_tag": "2",
            "duration_target_min": 5,
            "peer_id": "peer-android",
            "event_type": "first_detect",
            "rssi": -78,
            "battery_pct": 66,
        },
    ]

    with ios_log.open("w", encoding="utf-8") as handle:
        for event in ios_events:
            handle.write(json.dumps(event) + "\n")

    with android_log.open("w", encoding="utf-8") as handle:
        for event in android_events:
            handle.write(json.dumps(event) + "\n")

    return [ios_log, android_log]


def main() -> int:
    parser = argparse.ArgumentParser(description="Summarize SPIKE-0003 BLE jsonl logs.")
    parser.add_argument("paths", nargs="*", help="jsonl files or directories")
    parser.add_argument(
        "--out-dir",
        default=str(Path(__file__).resolve().parent / "out"),
        help="output directory for summary.md and summary.csv",
    )
    parser.add_argument(
        "--demo",
        action="store_true",
        help="generate demo logs and run the summary on them",
    )
    args = parser.parse_args()

    out_dir = Path(args.out_dir)
    out_dir.mkdir(parents=True, exist_ok=True)

    input_paths: List[Path] = []
    if args.demo:
        if args.paths:
            print("[error] --demo cannot be combined with explicit paths", flush=True)
            return 2
        input_paths = write_demo_logs(out_dir)
    else:
        input_paths = gather_files(args.paths)

    if not input_paths:
        print("[error] no input files provided", flush=True)
        return 2

    trials: Dict[Tuple[str, str, str, str], Dict[str, Any]] = {}
    line_counts: Dict[str, int] = {}
    total_lines = 0
    parsed_lines = 0
    parse_errors = 0
    missing_trial_id = 0
    missing_ts = 0
    negative_t_detect = 0

    for path in input_paths:
        line_counts[str(path)] = 0
        try:
            with path.open("r", encoding="utf-8") as handle:
                for raw_line in handle:
                    total_lines += 1
                    line = raw_line.strip()
                    if not line:
                        continue
                    line_counts[str(path)] += 1
                    try:
                        record = json.loads(line)
                    except json.JSONDecodeError:
                        parse_errors += 1
                        continue
                    parsed_lines += 1

                    trial_id = record.get("trial_id")
                    if not trial_id:
                        missing_trial_id += 1
                        continue

                    os_name = normalize_str(record.get("os"))
                    device_role = normalize_str(record.get("device_role"))
                    peer_id = normalize_str(record.get("peer_id"))
                    trial_key = (trial_id, os_name, device_role, peer_id)

                    trial = trials.setdefault(
                        trial_key,
                        {
                            "trial_id": trial_id,
                            "os": os_name,
                            "device_role": device_role,
                            "peer_id": peer_id,
                            "start_ts": None,
                            "start_record": None,
                            "first_detect_ts": None,
                            "first_detect_record": None,
                            "first_event_ts": None,
                            "first_event_record": None,
                            "source_files": set(),
                        },
                    )

                    trial["source_files"].add(str(path))
                    ts = parse_ts(record.get("ts"))
                    if ts is None:
                        missing_ts += 1
                        continue

                    if trial["first_event_ts"] is None or ts < trial["first_event_ts"]:
                        trial["first_event_ts"] = ts
                        trial["first_event_record"] = record

                    event_type = record.get("event_type")
                    if event_type == "trial_start":
                        if trial["start_ts"] is None or ts < trial["start_ts"]:
                            trial["start_ts"] = ts
                            trial["start_record"] = record
                    elif event_type == "first_detect":
                        if trial["first_detect_ts"] is None or ts < trial["first_detect_ts"]:
                            trial["first_detect_ts"] = ts
                            trial["first_detect_record"] = record
        except FileNotFoundError:
            print(f"[warn] missing file: {path}", flush=True)
            continue

    groups: Dict[Tuple[str, str, str, str, str, str], Dict[str, Any]] = {}
    raw_rows: List[Dict[str, Any]] = []
    missing_trial_start = 0

    for trial in trials.values():
        if trial["start_ts"] is None:
            missing_trial_start += 1
            continue

        base_record = trial["start_record"] or trial["first_event_record"]
        if base_record is None:
            missing_trial_start += 1
            continue

        state_record = trial["first_detect_record"] or base_record

        proximity_tag = normalize_str(base_record.get("proximity_tag"))
        distance_tag = normalize_number(base_record.get("distance_m_tag"))
        duration_tag = normalize_number(base_record.get("duration_target_min"))
        app_state = normalize_str(state_record.get("app_state"))
        low_power = normalize_low_power(state_record)
        os_name = normalize_str(base_record.get("os"))

        duration_min = coerce_float(base_record.get("duration_target_min"))

        t_detect_sec = None
        if trial["first_detect_ts"] is not None:
            delta = (trial["first_detect_ts"] - trial["start_ts"]).total_seconds()
            if delta >= 0:
                t_detect_sec = delta
            else:
                negative_t_detect += 1

        success_within_target = None
        if duration_min is not None:
            if t_detect_sec is not None:
                success_within_target = t_detect_sec <= duration_min * 60.0
            else:
                success_within_target = False

        group_key = (
            os_name,
            proximity_tag,
            duration_tag,
            distance_tag,
            app_state,
            low_power,
        )
        group = groups.setdefault(
            group_key,
            {
                "trials": 0,
                "trials_with_target": 0,
                "success_within_target": 0,
                "t_detect_sec": [],
            },
        )
        group["trials"] += 1
        if duration_min is not None:
            group["trials_with_target"] += 1
            if success_within_target:
                group["success_within_target"] += 1
        if t_detect_sec is not None:
            group["t_detect_sec"].append(t_detect_sec)

        raw_rows.append(
            {
                "trial_id": trial["trial_id"],
                "os": os_name,
                "device_role": trial["device_role"],
                "peer_id": trial["peer_id"],
                "proximity_tag": proximity_tag,
                "distance_m_tag": distance_tag,
                "duration_target_min": duration_tag,
                "duration_target_min_num": "" if duration_min is None else duration_min,
                "app_state": app_state,
                "low_power": low_power,
                "trial_start_ts": trial["start_ts"].isoformat(),
                "first_detect_ts": ""
                if trial["first_detect_ts"] is None
                else trial["first_detect_ts"].isoformat(),
                "t_detect_sec": "" if t_detect_sec is None else round(t_detect_sec, 3),
                "t_detect_min": ""
                if t_detect_sec is None
                else round(t_detect_sec / 60.0, 3),
                "success_within_target": ""
                if success_within_target is None
                else str(success_within_target).lower(),
                "source_files": ";".join(sorted(trial["source_files"])),
            }
        )

    summary_md = out_dir / "summary.md"
    summary_csv = out_dir / "summary.csv"

    with summary_csv.open("w", newline="", encoding="utf-8") as handle:
        fieldnames = [
            "trial_id",
            "os",
            "device_role",
            "peer_id",
            "proximity_tag",
            "distance_m_tag",
            "duration_target_min",
            "duration_target_min_num",
            "app_state",
            "low_power",
            "trial_start_ts",
            "first_detect_ts",
            "t_detect_sec",
            "t_detect_min",
            "success_within_target",
            "source_files",
        ]
        writer = csv.DictWriter(handle, fieldnames=fieldnames)
        writer.writeheader()
        for row in raw_rows:
            writer.writerow(row)

    with summary_md.open("w", encoding="utf-8") as handle:
        handle.write("# SPIKE-0003 summary\n\n")
        handle.write(f"Generated: {datetime.now(timezone.utc).isoformat()}\n\n")

        handle.write("## Inputs\n")
        for path, count in sorted(line_counts.items()):
            handle.write(f"- {path} ({count} lines)\n")
        handle.write("\n")

        handle.write("## Parse notes\n")
        handle.write(f"- total_lines: {total_lines}\n")
        handle.write(f"- parsed_lines: {parsed_lines}\n")
        handle.write(f"- parse_errors: {parse_errors}\n")
        handle.write(f"- missing_trial_id: {missing_trial_id}\n")
        handle.write(f"- missing_ts: {missing_ts}\n")
        handle.write(f"- missing_trial_start: {missing_trial_start}\n")
        handle.write(f"- negative_t_detect: {negative_t_detect}\n\n")

        handle.write(
            "Success = first_detect occurs within duration_target_min minutes.\n"
        )
        handle.write("P(N) denominator uses trials with numeric duration_target_min.\n\n")

        headers = [
            "os",
            "proximity_tag",
            "duration_target_min",
            "distance_m_tag",
            "app_state",
            "low_power",
            "trials",
            "success_within_target",
            "P(N)",
            "t_detect_n",
            "t_detect_min",
            "t_detect_median",
            "t_detect_mean",
            "t_detect_max",
        ]
        handle.write("## Group summary\n")
        handle.write("| " + " | ".join(headers) + " |\n")
        handle.write("| " + " | ".join(["---"] * len(headers)) + " |\n")

        for key in sorted(groups.keys()):
            group = groups[key]
            t_values = [v / 60.0 for v in group["t_detect_sec"]]
            if t_values:
                t_min = f"{min(t_values):.2f}"
                t_median = f"{statistics.median(t_values):.2f}"
                t_mean = f"{statistics.mean(t_values):.2f}"
                t_max = f"{max(t_values):.2f}"
            else:
                t_min = t_median = t_mean = t_max = "-"

            row = list(key) + [
                str(group["trials"]),
                str(group["success_within_target"]),
                format_ratio(
                    group["success_within_target"], group["trials_with_target"]
                ),
                str(len(t_values)),
                t_min,
                t_median,
                t_mean,
                t_max,
            ]
            handle.write("| " + " | ".join(row) + " |\n")

        handle.write("\n")
        handle.write(f"Raw rows: {summary_csv}\n")

    print(f"[ok] wrote {summary_md} and {summary_csv}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
