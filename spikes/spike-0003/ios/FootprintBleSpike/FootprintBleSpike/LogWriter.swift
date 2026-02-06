import Foundation

final class LogWriter {
    let fileURL: URL
    private let queue = DispatchQueue(label: "fp.ble.log-writer")

    init(filename: String = "ble_spike.jsonl") {
        let directory = FileManager.default.urls(for: .documentDirectory, in: .userDomainMask).first!
        fileURL = directory.appendingPathComponent(filename)
        if !FileManager.default.fileExists(atPath: fileURL.path) {
            FileManager.default.createFile(atPath: fileURL.path, contents: nil, attributes: nil)
        }
    }

    func append(_ record: [String: Any]) {
        queue.async {
            do {
                let jsonData = try JSONSerialization.data(withJSONObject: record, options: [])
                guard var jsonLine = String(data: jsonData, encoding: .utf8) else {
                    return
                }
                jsonLine.append("\n")
                guard let lineData = jsonLine.data(using: .utf8) else {
                    return
                }
                let handle = try FileHandle(forWritingTo: self.fileURL)
                handle.seekToEndOfFile()
                handle.write(lineData)
                handle.closeFile()
            } catch {
                print("[log] write failed: \(error)")
            }
        }
    }
}
