import Foundation

enum Colours: String {
    case red     = "\u{001B}[31m"
    case green   = "\u{001B}[32m"
    case yellow  = "\u{001B}[33m"
    case blue    = "\u{001B}[34m"
    case magenta = "\u{001B}[35m"
    case cyan    = "\u{001B}[36m"
    case reset   = "\u{001B}[0m"
}
