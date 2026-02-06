Added a minimal SwiftUI BLE spike app isolated under `spikes/spike-0003/ios/FootprintBleSpike/` with simultaneous advertise/scan, JSONL logging, and ShareSheet export, plus an Xcode project and Info.plist background modes; no changes to `./ci.sh`. Details live in `artifacts/runs/TASK-0310-agent2-spike-0003-ios.out.md`.

- BLE central/peripheral logic, trial logging, and first_detect/detect handling are in `spikes/spike-0003/ios/FootprintBleSpike/FootprintBleSpike/BleManager.swift`.
- Settings UI, trial controls, and share button are in `spikes/spike-0003/ios/FootprintBleSpike/FootprintBleSpike/ContentView.swift`.
- JSONL append to Documents/`ble_spike.jsonl` is in `spikes/spike-0003/ios/FootprintBleSpike/FootprintBleSpike/LogWriter.swift`.
- Background modes + Bluetooth usage text are in `spikes/spike-0003/ios/FootprintBleSpike/FootprintBleSpike/Info.plist`.
- Xcode project scaffold is in `spikes/spike-0003/ios/FootprintBleSpike/FootprintBleSpike.xcodeproj/project.pbxproj`.

Tests/builds not run here (requires Xcode + device signing).

Next steps you can take:
1) Open `spikes/spike-0003/ios/FootprintBleSpike/FootprintBleSpike.xcodeproj` in Xcode, set signing, and run on a device.
2) Verify foreground scan/advertise + logging, then lock/home to observe background log growth.
3) Use “Share Logs” to export `ble_spike.jsonl` for analysis.