import Foundation

func saveURL() -> URL {
    let home = FileManager.default.homeDirectoryForCurrentUser
    let dir  = home.appendingPathComponent(".local/share/termfarm", isDirectory: true)

    try? FileManager.default.createDirectory(
        at: dir,
        withIntermediateDirectories: true
    )

    return dir.appendingPathComponent("save.json")
}

func loadFarm() throws -> FarmState {
    let url = saveURL()
        let data = try Data(contentsOf: url)

        return try JSONDecoder().decode(FarmState.self, from: data)
}

func saveFarm(_ farm: FarmState) throws {
    let data = try JSONEncoder().encode(farm)
    try data.write(to: saveURL(), options: .atomic)
}
