import Foundation

func formatDuration(_ seconds: TimeInterval) -> String {
    let s = Int(seconds)
    let h = s / 3600
    let m = (s % 3600) / 60

    if h > 0 {
        return "\(h)h \(m)m"
    } else {
        return "\(m)m"
    }
}
