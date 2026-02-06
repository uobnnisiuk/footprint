import CoreBluetooth
import SwiftUI

final class BleManager: NSObject, ObservableObject {
    static let serviceUUID = CBUUID(string: "6A8F0C6C-5ED6-4D2D-9EAD-1CC1E7E4F3A1")
    static let characteristicUUID = CBUUID(string: "7C2B68D9-79A8-4C33-8D84-25D2F70E2B4F")

    @Published private(set) var isScanning = false
    @Published private(set) var isAdvertising = false
    @Published private(set) var isTrialActive = false
    @Published private(set) var currentTrialId: String? = nil
    @Published private(set) var statusText: String = ""

    private let settings: SettingsStore
    private let logWriter: LogWriter
    private let dateFormatter: ISO8601DateFormatter
    private var centralManager: CBCentralManager!
    private var peripheralManager: CBPeripheralManager!
    private var firstDetectLogged = false
    private var didAddService = false
    private var appState: String = "foreground"

    var logFileURL: URL { logWriter.fileURL }

    init(settings: SettingsStore, logWriter: LogWriter) {
        self.settings = settings
        self.logWriter = logWriter
        let formatter = ISO8601DateFormatter()
        formatter.formatOptions = [.withInternetDateTime, .withFractionalSeconds]
        dateFormatter = formatter
        super.init()

        UIDevice.current.isBatteryMonitoringEnabled = true

        centralManager = CBCentralManager(
            delegate: self,
            queue: nil,
            options: [
                CBCentralManagerOptionRestoreIdentifierKey: "fp.ble.central",
                CBCentralManagerOptionShowPowerAlertKey: true
            ]
        )

        peripheralManager = CBPeripheralManager(
            delegate: self,
            queue: nil,
            options: [
                CBPeripheralManagerOptionRestoreIdentifierKey: "fp.ble.peripheral"
            ]
        )
    }

    func updateAppState(_ phase: ScenePhase) {
        switch phase {
        case .active, .inactive:
            appState = "foreground"
        case .background:
            appState = "background"
        @unknown default:
            appState = "foreground"
        }
    }

    func startTrial() {
        let trialId = UUID().uuidString
        currentTrialId = trialId
        isTrialActive = true
        firstDetectLogged = false
        logEvent(eventType: "trial_start", trialId: trialId, rssi: nil)
        statusText = "trial started"
    }

    func endTrial() {
        guard let trialId = currentTrialId, isTrialActive else { return }
        logEvent(eventType: "trial_end", trialId: trialId, rssi: nil)
        isTrialActive = false
        currentTrialId = nil
        firstDetectLogged = false
        statusText = "trial ended"
    }

    private func logEvent(eventType: String, trialId: String, rssi: Int?) {
        var record: [String: Any] = [
            "ts": dateFormatter.string(from: Date()),
            "trial_id": trialId,
            "device_role": settings.deviceRole,
            "os": "iOS",
            "app_state": appState,
            "low_power": ProcessInfo.processInfo.isLowPowerModeEnabled,
            "proximity_tag": settings.proximityTag,
            "distance_m_tag": settings.distanceTag,
            "duration_target_min": settings.durationTargetMin,
            "peer_id": settings.peerId,
            "event_type": eventType,
            "battery_pct": currentBatteryPercent()
        ]

        if let rssi = rssi {
            record["rssi"] = rssi
        }

        logWriter.append(record)
    }

    private func currentBatteryPercent() -> Int {
        let level = UIDevice.current.batteryLevel
        if level < 0 {
            return -1
        }
        return Int((level * 100.0).rounded())
    }

    private func startScanning() {
        guard centralManager.state == .poweredOn else { return }
        if centralManager.isScanning {
            return
        }
        centralManager.scanForPeripherals(
            withServices: [Self.serviceUUID],
            options: [CBCentralManagerScanOptionAllowDuplicatesKey: true]
        )
        isScanning = true
    }

    private func startAdvertising() {
        guard peripheralManager.state == .poweredOn else { return }
        if peripheralManager.isAdvertising {
            return
        }
        let localName = "FP-" + String(settings.peerId.prefix(4))
        let advertisement: [String: Any] = [
            CBAdvertisementDataServiceUUIDsKey: [Self.serviceUUID],
            CBAdvertisementDataLocalNameKey: String(localName)
        ]
        peripheralManager.startAdvertising(advertisement)
    }
}

extension BleManager: CBCentralManagerDelegate {
    func centralManagerDidUpdateState(_ central: CBCentralManager) {
        switch central.state {
        case .poweredOn:
            startScanning()
            statusText = "central on"
        case .poweredOff:
            isScanning = false
            statusText = "central off"
        case .unauthorized:
            isScanning = false
            statusText = "central unauthorized"
        case .unsupported:
            isScanning = false
            statusText = "central unsupported"
        default:
            isScanning = false
            statusText = "central state \(central.state.rawValue)"
        }
    }

    func centralManager(_ central: CBCentralManager, didDiscover peripheral: CBPeripheral, advertisementData: [String: Any], rssi RSSI: NSNumber) {
        guard isTrialActive, let trialId = currentTrialId else { return }
        let rssiValue = RSSI.intValue
        if !firstDetectLogged {
            logEvent(eventType: "first_detect", trialId: trialId, rssi: rssiValue)
            firstDetectLogged = true
        } else {
            logEvent(eventType: "detect", trialId: trialId, rssi: rssiValue)
        }
    }
}

extension BleManager: CBPeripheralManagerDelegate {
    func peripheralManagerDidUpdateState(_ peripheral: CBPeripheralManager) {
        switch peripheral.state {
        case .poweredOn:
            if !didAddService {
                let characteristic = CBMutableCharacteristic(
                    type: Self.characteristicUUID,
                    properties: [.read],
                    value: nil,
                    permissions: [.readable]
                )
                let service = CBMutableService(type: Self.serviceUUID, primary: true)
                service.characteristics = [characteristic]
                peripheral.add(service)
            } else {
                startAdvertising()
            }
            statusText = "peripheral on"
        case .poweredOff:
            isAdvertising = false
            statusText = "peripheral off"
        case .unauthorized:
            isAdvertising = false
            statusText = "peripheral unauthorized"
        case .unsupported:
            isAdvertising = false
            statusText = "peripheral unsupported"
        default:
            isAdvertising = false
            statusText = "peripheral state \(peripheral.state.rawValue)"
        }
    }

    func peripheralManager(_ peripheral: CBPeripheralManager, didAdd service: CBService, error: Error?) {
        if let error = error {
            statusText = "service add error: \(error.localizedDescription)"
            return
        }
        didAddService = true
        startAdvertising()
    }

    func peripheralManagerDidStartAdvertising(_ peripheral: CBPeripheralManager, error: Error?) {
        if let error = error {
            isAdvertising = false
            statusText = "advertise error: \(error.localizedDescription)"
            return
        }
        isAdvertising = true
        statusText = "advertising"
    }

    func peripheralManager(_ peripheral: CBPeripheralManager, didReceiveRead request: CBATTRequest) {
        guard request.characteristic.uuid == Self.characteristicUUID else {
            peripheral.respond(to: request, withResult: .requestNotSupported)
            return
        }
        if let value = settings.peerId.data(using: .utf8) {
            request.value = value
            peripheral.respond(to: request, withResult: .success)
        } else {
            peripheral.respond(to: request, withResult: .unlikelyError)
        }
    }
}
