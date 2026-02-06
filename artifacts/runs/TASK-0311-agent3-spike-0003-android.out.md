Built a self‑contained Android BLE spike app under the requested spike path that runs a foreground service to advertise+scan a fixed Service UUID and appends JSONL to internal storage, with a minimal UI to start/end trials, toggle the service, and share logs. I isolated everything in the new spike project so `./ci.sh` stays untouched, and wired the log fields + permissions directly in the app/service code to match the required JSONL shape.

Details and key paths:
- Gradle project root: `spikes/spike-0003/android/FootprintBleSpike`
- BLE service + logging: `spikes/spike-0003/android/FootprintBleSpike/app/src/main/java/com/footprint/ble/spike/BleForegroundService.kt`, `spikes/spike-0003/android/FootprintBleSpike/app/src/main/java/com/footprint/ble/spike/BleLogger.kt`
- UI + prefs: `spikes/spike-0003/android/FootprintBleSpike/app/src/main/java/com/footprint/ble/spike/MainActivity.kt`, `spikes/spike-0003/android/FootprintBleSpike/app/src/main/java/com/footprint/ble/spike/Prefs.kt`
- Manifest + permissions + FileProvider: `spikes/spike-0003/android/FootprintBleSpike/app/src/main/AndroidManifest.xml`, `spikes/spike-0003/android/FootprintBleSpike/app/src/main/res/xml/file_paths.xml`
- Run notes: `artifacts/runs/TASK-0311-agent3-spike-0003-android.out.md`

Tests not run (Android SDK/device not available in this environment).

Note: `git status` shows an untracked file I didn’t create (`artifacts/runs/TASK-0310-agent2-spike-0003-ios.out.md`). I left it untouched—how do you want to handle it?

Next steps:
1) Open `spikes/spike-0003/android/FootprintBleSpike` in Android Studio and run on a physical device to confirm foreground + lock-screen logging.
2) Exercise TST-0311-2 by locking the screen for a few minutes and confirm `ble_spike.jsonl` grows via “Share Logs.”