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
        ]
    )
}
