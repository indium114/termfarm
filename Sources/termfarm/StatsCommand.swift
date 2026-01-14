import ArgumentParser
import Foundation

struct Stats: ParsableCommand {
    static let configuration = CommandConfiguration(
        abstract: "Show farm statistics"
    )

    @Flag(name: .long)
    var json = false

    func run() throws {
        let farm  = try loadFarm()
        let stats = computeStats(farm: farm)

        if json {
            let encoder = JSONEncoder()
            encoder.outputFormatting = [.sortedKeys]
            let data = try encoder.encode(stats)
            FileHandle.standardOutput.write(data)
        } else {
            let trendIcon = stats.marketTrend > 0 ? "󰔵" :
                            stats.marketTrend < 0 ? "󰔳" : "󰔴"

            let trendPct  = String(format: "%+.1f%%", stats.marketTrend * 100) 

            print("""
             \(stats.readyToHarvest)/\(stats.totalPlots) ready \
            󰜦 \(stats.inventoryCrops) crops in inventory
            󰹢 \(stats.inventorySeeds) seeds in inventory
             \(stats.coins) coins in wallet
            \(trendIcon) \(trendPct)
            """)
        }
    }
}
