import SwiftUI

struct ContentView: View {
    @ObservedObject var manager: BleManager
    @ObservedObject var settings: SettingsStore
    @Environment(\.scenePhase) private var scenePhase
    @State private var isSharePresented = false

    var body: some View {
        NavigationView {
            Form {
                Section(header: Text("Trial")) {
                    HStack {
                        Text("Trial ID")
                        Spacer()
                        Text(manager.currentTrialId ?? "-")
                            .font(.footnote)
                            .foregroundColor(.secondary)
                    }
                    Button("Start Trial") {
                        manager.startTrial()
                    }
                    .disabled(manager.isTrialActive)

                    Button("End Trial") {
                        manager.endTrial()
                    }
                    .disabled(!manager.isTrialActive)
                }

                Section(header: Text("Settings")) {
                    Picker("Device Role", selection: $settings.deviceRole) {
                        Text("A").tag("A")
                        Text("B").tag("B")
                    }

                    Picker("Proximity", selection: $settings.proximityTag) {
                        Text("room").tag("room")
                        Text("floor").tag("floor")
                        Text("building").tag("building")
                        Text("other").tag("other")
                    }

                    Picker("Distance (m)", selection: $settings.distanceTag) {
                        Text("1").tag("1")
                        Text("3").tag("3")
                        Text("10").tag("10")
                        Text("other").tag("other")
                    }

                    Picker("Duration (min)", selection: $settings.durationTargetMin) {
                        Text("5").tag("5")
                        Text("30").tag("30")
                        Text("120").tag("120")
                        Text("other").tag("other")
                    }
                }

                Section(header: Text("Status")) {
                    HStack {
                        Text("Scanning")
                        Spacer()
                        Text(manager.isScanning ? "on" : "off")
                            .foregroundColor(manager.isScanning ? .green : .secondary)
                    }
                    HStack {
                        Text("Advertising")
                        Spacer()
                        Text(manager.isAdvertising ? "on" : "off")
                            .foregroundColor(manager.isAdvertising ? .green : .secondary)
                    }
                    if !manager.statusText.isEmpty {
                        Text(manager.statusText)
                            .font(.footnote)
                            .foregroundColor(.secondary)
                    }
                }

                Section(header: Text("Logs")) {
                    Button("Share Logs") {
                        isSharePresented = true
                    }
                }
            }
            .navigationTitle("BLE Spike")
        }
        .sheet(isPresented: $isSharePresented) {
            ShareSheet(activityItems: [manager.logFileURL])
        }
        .onChange(of: scenePhase) { newPhase in
            manager.updateAppState(newPhase)
        }
        .onAppear {
            manager.updateAppState(scenePhase)
        }
    }
}
