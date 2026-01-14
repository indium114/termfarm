import ArgumentParser

struct TermFarm: ParsableCommand {
    static let configuration = CommandConfiguration(
        commandName: "termfarm",
        subcommands: [
            Init.self,
            Stats.self
        ]
    )
}
