import ArgumentParser
import Foundation

struct Buy: ParsableCommand {
    static let configuration = CommandConfiguration(
        abstract: "Buy seeds from the market"
    )

    @Argument(help: "Seed Name (e.g. 'wheat')")
    var seedID: String

    @Argument(help: "Amount to buy")
    var amount: Int

    func run() throws {
        guard amount > 0 else {
            throw ValidationError("Amount must be greater than zero")
        }

        var farm = try loadFarm()

        guard let crop = cropRegistry[seedID] else {
            throw ValidationError("Unknown seed: '\(seedID)'")
        }

        guard farm.market.availableSeeds.contains(seedID) else {
            throw ValidationError("Seed '\(seedID)' is not available on the market")
        }

        guard let unitPrice = buyPrice(for: seedID, farm: farm) else {
            throw ValidationError("Price unavailable for seed '\(seedID)'")
        }

        let totalCost = unitPrice * amount

        guard farm.coins >= totalCost else {
            throw ValidationError("Not enough coins. You need \(totalCost), but only have \(farm.coins)")
        }

        farm.coins -= totalCost
        farm.inventory.seeds[seedID, default: 0] += amount
        farm.lastUpdated = Date()

        try saveFarm(farm)

        print("󰄐 Bought \(amount)x \(seedID) seeds for \(totalCost) coins (\(unitPrice) each)")
    }
}
