import ArgumentParser
import Foundation

struct Init: ParsableCommand {
    static let configuration = CommandConfiguration(
        abstract: "Initialise a new Termfarm save file"
    )

    func run() throws {
        let farm = FarmState(
            coins: 100,
            plots: (0..<3).map { _ in Plot(id: UUID(), plantedCrop: nil, plantedAt: nil) },
            inventory: Inventory(),
            market: MarketState(
                availableSeeds: [ "wheat", "carrot" ],
                priceModifiers: [:],
                lastRotation: Date()
            ),
            lastUpdated: Date()
        )

        try saveFarm(farm)
        print("termfarm save file initialised!") // TODO: Add a nerd font icon
    }
}
