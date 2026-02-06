import Foundation

final class SettingsStore: ObservableObject {
    private enum Keys {
        static let peerId = "fp.peer_id"
        static let deviceRole = "fp.device_role"
        static let proximityTag = "fp.proximity_tag"
        static let distanceTag = "fp.distance_m_tag"
        static let durationTarget = "fp.duration_target_min"
    }

    @Published var deviceRole: String {
        didSet { defaults.set(deviceRole, forKey: Keys.deviceRole) }
    }

    @Published var proximityTag: String {
        didSet { defaults.set(proximityTag, forKey: Keys.proximityTag) }
    }

    @Published var distanceTag: String {
        didSet { defaults.set(distanceTag, forKey: Keys.distanceTag) }
    }

    @Published var durationTargetMin: String {
        didSet { defaults.set(durationTargetMin, forKey: Keys.durationTarget) }
    }

    let peerId: String

    private let defaults = UserDefaults.standard

    init() {
        if let stored = defaults.string(forKey: Keys.peerId) {
            peerId = stored
        } else {
            let generated = UUID().uuidString
            defaults.set(generated, forKey: Keys.peerId)
            peerId = generated
        }

        deviceRole = defaults.string(forKey: Keys.deviceRole) ?? "A"
        proximityTag = defaults.string(forKey: Keys.proximityTag) ?? "room"
        distanceTag = defaults.string(forKey: Keys.distanceTag) ?? "1"
        durationTargetMin = defaults.string(forKey: Keys.durationTarget) ?? "5"
    }
}
