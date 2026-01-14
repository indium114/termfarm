import ArgumentParser
import Foundation

struct InventoryCommand: ParsableCommand {
    static let configuration = CommandConfiguration(
        commandName: "inventory",
        abstract: "List the contents of your inventory"
    )

    func run() throws {
        let farm = try loadFarm()

        print("󰜦 Inventory:")
        print("------------")
        print("")
        print(" Coins: \(farm.coins)")
        print("󰹢 Seeds:")

        if farm.inventory.seeds.isEmpty {
            print(" none")
        } else {
            for (seed, amount) in farm.inventory.seeds.sorted(by: { $0.key < $1.key }) {
                print(" - \(amount)x \(seed)")
            }
        }

        print("")
        print(" Crops:")

        if farm.inventory.crops.isEmpty {
            print(" none")
        } else {
            for (crop, amount) in farm.inventory.crops.sorted(by: { $0.key < $1.key }) {
                print(" - \(amount)x \(crop)")
            }
        }
    }
}
