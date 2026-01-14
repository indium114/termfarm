import Foundation

let marketRotationInterval: TimeInterval = 4 * 60 * 60
let marketMaxItems = 3

let priceModifierRange: ClosedRange<Double> = 0.7...1.3

func generateMarket() -> MarketState {
    let allSeeds = Array(cropRegistry.keys).shuffled()
    let selection = Array(allSeeds.prefix(marketMaxItems))

    var modifiers: [String: Double] = [:]
    for seed in selection {
        modifiers[seed] = Double.random(in: priceModifierRange)
    }

    return MarketState(
        availableSeeds: selection,
        priceModifiers: modifiers,
        lastRotation: Date()
    )
}

func updateMarketIfNeeded(farm: inout FarmState) {
    let now = Date()

    guard now.timeIntervalSince(farm.market.lastRotation) >= marketRotationInterval else {
        return
    }

    farm.market = generateMarket()

    // Pick random seeds from registry
    let allSeeds = Array(cropRegistry.keys).shuffled()
    let selection = Array(allSeeds.prefix(marketMaxItems))

    var modifiers: [String: Double] = [:]
    for seed in selection {
        modifiers[seed] = Double.random(in: priceModifierRange)
    }

    farm.market = MarketState(
        availableSeeds: selection,
        priceModifiers: modifiers,
        lastRotation: now
    )
}

func buyPrice(for cropID: String, farm: FarmState) -> Int? {
    guard
        let crop     = cropRegistry[cropID],
        let modifier = farm.market.priceModifiers[cropID]
    else { return nil }

    return Int(Double(crop.baseBuyPrice) * modifier)
}

func sellPrice(for cropID: String, farm: FarmState) -> Int {
    let crop     = cropRegistry[cropID]!
    let modifier = farm.market.priceModifiers[cropID] ?? 1.0
    return Int(Double(crop.baseSellPrice) * modifier)
}
