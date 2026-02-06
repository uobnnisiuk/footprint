import SwiftUI

@main
struct FootprintBleSpikeApp: App {
    @StateObject private var settings: SettingsStore
    @StateObject private var manager: BleManager

    init() {
        let settings = SettingsStore()
        let logWriter = LogWriter()
        _settings = StateObject(wrappedValue: settings)
        _manager = StateObject(wrappedValue: BleManager(settings: settings, logWriter: logWriter))
    }

    var body: some Scene {
        WindowGroup {
            ContentView(manager: manager, settings: settings)
        }
    }
}
