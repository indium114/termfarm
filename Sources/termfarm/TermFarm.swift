import ArgumentParser

struct TermFarm: ParsableCommand {
    static let configuration = CommandConfiguration(
        commandName: "termfarm",
        abstract: "A terminal idle farming game",
        subcommands: [
            Init.self,
            Stats.self,
            Market.self,
            Buy.self,
            Plant.self,
            Harvest.self,
            InventoryCommand.self,
            Sell.self,
            BuyPlotCommand.self,
            View.self
        ]
    )
}
