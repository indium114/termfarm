import Foundation

struct FarmStats: Codable {
    let readyToHarvest: Int
    let growingCrops: Int
    let totalPlots: Int
    let inventoryCrops: Int
    let inventorySeeds: Int
    let coins: Int
    let marketItems: Int
    let marketTrend: Double
    let nextMarketRotationIn: TimeInterval
}

func computeStats(farm: FarmState) -> FarmStats {
    let now     = Date()
    var ready   = 0
    var growing = 0

    for plot in farm.plots {
        guard 
            let cropID    = plot.plantedCrop,
            let plantedAt = plot.plantedAt,
            let crop      = cropRegistry[cropID]
        else { continue }

        if now.timeIntervalSince(plantedAt) >= crop.growTime {
            ready += 1
        } else {
            growing += 1
        }
    }

    let crops = farm.inventory.crops.values.reduce(0, +)
    let seeds = farm.inventory.seeds.values.reduce(0, +)

    let trend: Double = {
        let values = farm.market.priceModifiers.values
        guard !values.isEmpty else { return 0 }
        return values.reduce(0, +) / Double(values.count) - 1
    }()

    let rotationInterval: TimeInterval = 4 * 60 * 60
    let remaining = max(
        0,
        rotationInterval - now.timeIntervalSince(farm.market.lastRotation)
    )

    return FarmStats(
        readyToHarvest: ready,
        growingCrops: growing,
        totalPlots: farm.plots.count,
        inventoryCrops: crops,
        inventorySeeds: seeds,
        coins: farm.coins,
        marketItems: farm.market.availableSeeds.count,
        marketTrend: trend,
        nextMarketRotationIn: remaining
    )
}
