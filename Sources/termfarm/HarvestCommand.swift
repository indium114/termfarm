import ArgumentParser
import Foundation

struct Harvest: ParsableCommand {
    static let configuration = CommandConfiguration(
        abstract: "Harvest all mature crops"
    )

    func run() throws {
        var farm = try loadFarm()
        let now  = Date()

        var harvested: [String: Int] = [:]

        for index in farm.plots.indices {
            guard
                let cropID    = farm.plots[index].plantedCrop,
                let plantedAt = farm.plots[index].plantedAt,
                let crop      = cropRegistry[cropID]
            else { continue }

            let growTime = crop.growTime
            let age      = now.timeIntervalSince(plantedAt)

            guard age >= growTime else { continue }

            farm.inventory.crops[cropID, default: 0] += 1
            harvested[cropID, default: 0] += 1

            farm.plots[index].plantedCrop = nil
            farm.plots[index].plantedAt   = nil
        }

        guard !harvested.isEmpty else {
            print(" No crops to harvest")
            return
        }

        farm.lastUpdated = now
        try saveFarm(farm)

        print("󱕓 Harvested:")
        for (crop, amount) in harvested {
            print(" +\(amount) \(crop)")
        }
    }
}
