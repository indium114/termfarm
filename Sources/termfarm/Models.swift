import Foundation

struct CropType: Codable, Identifiable {
    let id: String
    let growTime: TimeInterval
    let baseBuyPrice: Int
    let baseSellPrice: Int
}

struct Plot: Codable, Identifiable {
    let id: UUID
    var plantedCrop: String?
    var plantedAt: Date?
}

struct Inventory: Codable {
    var crops: [String: Int] = [:]
    var seeds: [String: Int] = [:]
}

struct MarketState: Codable {
    var availableSeeds: [String]
    var priceModifiers: [String: Double]
    var lastRotation: Date
}

struct FarmState: Codable {
    var coins: Int
    var plots: [Plot]
    var inventory: Inventory
    var market: MarketState
    var lastUpdated: Date
}
