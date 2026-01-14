import ArgumentParser
import Foundation

struct Sell: ParsableCommand {
    static let configuration = CommandConfiguration(
        abstract: "Sell crops from your inventory"
    )

    @Argument(help: "Crop to sell")
    var cropID: String

    @Argument(help: "Amount to sell (defaults to all)", transform: Int.init)
    var amount: Int?

    func run() throws {
        var farm = try loadFarm()

        guard let owned = farm.inventory.crops[cropID], owned > 0 else {
            throw ValidationError("You do not have any '\(cropID)' to sell")
        }

        let sellAmount = amount ?? owned
        guard sellAmount > 0 else {
            throw ValidationError("Invalid amount")
        }

        guard sellAmount <= owned else {
            throw ValidationError("You only have \(owned) \(cropID)")
        }

        let price = sellPrice(for: cropID, farm: farm)

        let total = price * sellAmount

        if farm.inventory.crops[cropID] == 0 {
            farm.inventory.crops.removeValue(forKey: cropID)
        }

        farm.coins      += total
        farm.lastUpdated = Date()
        try saveFarm(farm)

        print("Sold \(sellAmount)x \(cropID) for \(total) coins (\(price) each)")
    }
}
