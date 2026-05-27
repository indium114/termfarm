package internal

var cropRegistry = map[string]CropType{
	"wheat": CropType{
		Icon:          "🌾",
		ID:            "wheat",
		GrowTime:      60 * 5,
		BaseBuyPrice:  5,
		BaseSellPrice: 8,
	},
	"carrot": CropType{
		Icon:          "🥕",
		ID:            "carrot",
		GrowTime:      60 * 8,
		BaseBuyPrice:  8,
		BaseSellPrice: 13,
	},
	"corn": CropType{
		Icon:          "🌽",
		ID:            "corn",
		GrowTime:      60 * 7,
		BaseBuyPrice:  9,
		BaseSellPrice: 11,
	},
	"potato": CropType{
		Icon:          "🥔",
		ID:            "potato",
		GrowTime:      60 * 9,
		BaseBuyPrice:  3,
		BaseSellPrice: 4,
	},
	"tomato": CropType{
		Icon:          "🍅",
		ID:            "tomato",
		GrowTime:      60 * 7,
		BaseBuyPrice:  11,
		BaseSellPrice: 15,
	},
	"strawberry": CropType{
		Icon:          "🍓",
		ID:            "strawberry",
		GrowTime:      60 * 12,
		BaseBuyPrice:  20,
		BaseSellPrice: 30,
	},
	"grape": CropType{
		Icon:          "🍇",
		ID:            "grape",
		GrowTime:      60 * 15,
		BaseBuyPrice:  23,
		BaseSellPrice: 25,
	},
	"watermelon": CropType{
		Icon:          "🍉",
		ID:            "watermelon",
		GrowTime:      60 * 20,
		BaseBuyPrice:  30,
		BaseSellPrice: 35,
	},
	"broccoli": CropType{
		Icon:          "🥦",
		ID:            "broccoli",
		GrowTime:      60 * 8,
		BaseBuyPrice:  4,
		BaseSellPrice: 7,
	},
	"avocado": CropType{
		Icon:          "🥑",
		ID:            "avocado",
		GrowTime:      60 * 5,
		BaseBuyPrice:  6,
		BaseSellPrice: 9,
	},
}
