import Foundation

let cropRegistry: [String: CropType] = [
    "wheat": CropType(
        icon: "🌾",
        id: "wheat",
        growTime: 60 * 5,
        baseBuyPrice: 5,
        baseSellPrice: 8
    ),
    "carrot": CropType(
        icon: "🥕",
        id: "carrot",
        growTime: 60 * 8,
        baseBuyPrice: 8,
        baseSellPrice: 13
    ),
    "corn": CropType(
        icon: "🌽",
        id: "corn",
        growTime: 60 * 7,
        baseBuyPrice: 9,
        baseSellPrice: 11
    ),
    "potato": CropType(
        icon: "🥔",
        id: "potato",
        growTime: 60 * 9,
        baseBuyPrice: 3,
        baseSellPrice: 4
    ),
    "tomato": CropType(
        icon: "🍅",
        id: "tomato",
        growTime: 60 * 7,
        baseBuyPrice: 11,
        baseSellPrice: 15
    ),
    "strawberry": CropType(
        icon: "🍓",
        id: "strawberry",
        growTime: 60 * 12,
        baseBuyPrice: 20,
        baseSellPrice: 30
    ),
    "grape": CropType(
        icon: "🍇",
        id: "grape",
        growTime: 60 * 15,
        baseBuyPrice: 23,
        baseSellPrice: 25
    ),
    "watermelon": CropType(
        icon: "🍉",
        id: "watermelon",
        growTime: 60 * 20,
        baseBuyPrice: 30,
        baseSellPrice: 35
    )
]
