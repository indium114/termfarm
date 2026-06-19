import ArgumentParser
import Foundation

struct Market: ParsableCommand {
    static let configuration = CommandConfiguration(
        abstract: "View the seed market"
    )

    func run() throws {
        let farm = try loadFarm()
        let stats = computeStats(farm: farm)

        print("󰄐 Seed Market")
        print("-------------")
        print(" Balance: \(farm.coins)")
        print("󰑓 Rotates In: \(formatDuration(stats.nextMarketRotationIn))")
        print("-------------")
        print()

        for seed in farm.market.availableSeeds {
            let crop     = cropRegistry[seed]!
            let price    = buyPrice(for: seed, farm: farm)!
            let modifier = farm.market.priceModifiers[seed]! - 1

            let trend = modifier > 0 ? "󰔵" : modifier < 0 ? "󰔳" : "󰔴"
            let pct   = String(format: "%+.0f%%", modifier * 100)

            print("\(crop.icon) \(crop.id) - \(price) coins (\(trend) \(pct))")
        }
    }
}
