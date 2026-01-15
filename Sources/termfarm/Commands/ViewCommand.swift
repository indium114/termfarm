import ArgumentParser
import Foundation

struct View: ParsableCommand {
    static let configuration = CommandConfiguration(
        abstract: "View status of farm plots"
    )

    func run() throws {
        let farm = try loadFarm()
        let now  = Date()

        print("  Plots") // TODO: Add nerd font icon
        print("-------")

        for (index, plot) in farm.plots.enumerated() {
            let plotNumber = index + 1

            guard
                let cropID    = plot.plantedCrop,
                let plantedAt = plot.plantedAt,
                let crop      = cropRegistry[cropID]
            else {
                print("[\(plotNumber)] empty")
                continue
            }

            let elapsed   = now.timeIntervalSince(plantedAt)
            let remaining = crop.growTime - elapsed

            if remaining <= 0 {
                print("[\(plotNumber)] \(crop.icon) \(cropID) ready to harvest")
            } else {
                print("[\(plotNumber)] \(crop.icon) \(cropID) \(formatDuration(remaining)) left")
            }
        }
    }
}
