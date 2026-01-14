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
            \(Colours.green.rawValue) \(stats.readyToHarvest)/\(stats.totalPlots) ready \(Colours.reset.rawValue)| \
            \(Colours.blue.rawValue)󰜦 \(stats.inventoryCrops) crops in inventory \(Colours.reset.rawValue)| \
            \(Colours.cyan.rawValue)󰹢 \(stats.inventorySeeds) seeds in inventory \(Colours.reset.rawValue)| \
            \(Colours.yellow.rawValue) \(stats.coins) coins in wallet \(Colours.reset.rawValue)| \
            \(Colours.red.rawValue)\(trendIcon) \(trendPct) \(Colours.reset.rawValue)| \
            \(Colours.magenta.rawValue)  \(formatDuration(stats.nextMarketRotationIn)) until next market rotation\(Colours.reset.rawValue)
            """)
        }
    }
}
