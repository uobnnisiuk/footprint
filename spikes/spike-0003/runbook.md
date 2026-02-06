# SPIKE-0003 runbook: sustained proximity

This runbook standardizes how to run sustained proximity trials so multiple people can
reproduce the same conditions and collect comparable logs.

## Trial matrix (standard conditions)
- Same room: duration 5 minutes
- Same floor: duration 30 minutes
- Same building: duration 120 minutes

## Setup (both devices, before each condition)
1. Charge to >= 50% and enable Bluetooth.
2. Open the BLE Spike app.
3. Set Device Role so the two devices are different.
   - iOS: A / B
   - Android: alpha / beta
4. Set proximity_tag, distance_m_tag, duration_target_min to match the condition below.
5. Android: set the app Low Power toggle to match the OS Battery Saver state.
6. Start Trial on both devices within 30 seconds of each other.
7. Android only: tap "Start Service" to start BLE scanning/advertising.
8. iOS: confirm Status shows scanning/advertising on.

## Condition setup details

### Same room (5 minutes)
- Placement: same room, line-of-sight if possible, 1-3m separation.
- Tags:
  - iOS proximity_tag: room
  - Android proximity_tag: near
  - distance_m_tag: closest available option to the actual separation
  - duration_target_min: 5

### Same floor (30 minutes)
- Placement: same floor, different rooms or hallway, ~10-30m separation.
- Tags:
  - iOS proximity_tag: floor
  - Android proximity_tag: mid
  - distance_m_tag: closest available option to the actual separation
  - duration_target_min: 30

### Same building (120 minutes)
- Placement: same building, different floors or far ends, ~30-100m separation.
- Tags:
  - iOS proximity_tag: building
  - Android proximity_tag: far
  - distance_m_tag: closest available option to the actual separation
  - duration_target_min: 120

## One trial (step-by-step)
1. Set tags and duration on both devices.
2. Tap Start Trial on both devices.
3. Apply the desired app state (BG-LOCK or BG-LPM) per the OS steps below.
4. Leave devices in position for the full duration.
5. Unlock devices, return to the app, tap End Trial.
6. Export logs immediately and rename files with:
   - os, device_role, condition, date (example: ios-A-room-2025-02-06.jsonl)

## OS steps for BG-LOCK / BG-LPM (OS actions only)

### iOS
- BG-LOCK: press the side button to lock the screen; do not force-quit the app.
- BG-LPM: Settings > Battery > Low Power Mode = On (or Control Center toggle).

### Android
- BG-LOCK: press the power button to lock the screen.
- BG-LPM: Quick Settings > Battery Saver = On (or Settings > Battery > Battery Saver).

## Log collection
- iOS: tap "Share Logs" and save the `ble_spike.jsonl` file.
- Android: tap "Share Logs" and save the `ble_spike.jsonl` file.
- Logs are append-only. Use trial_id + timestamps to separate trials in analysis.
