import Foundation

let cropRegistry: [String: CropType] = [
    "wheat": CropType(
        id: "wheat",
        growTime: 60 * 5,
        baseBuyPrice: 5,
        baseSellPrice: 8
    ),
    "carrot": CropType(
        id: "carrot",
        growTime: 60 * 8,
        baseBuyPrice: 8,
        baseSellPrice: 13
    )
]
