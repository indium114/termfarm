package internal

import (
	"github.com/google/uuid"
	"time"
)

type CropType struct {
	Icon          string        `json:"icon"`
	ID            string        `json:"id"`
	GrowTime      time.Duration `json:"growTime"`
	BaseBuyPrice  int           `json:"baseBuyPrice"`
	BaseSellPrice int           `json:"baseSellPrice"`
}

type Plot struct {
	ID          uuid.UUID  `json:"id"`
	PlantedCrop *string    `json:"plantedCrop,omitempty"`
	PlantedAt   *time.Time `json:"plantedAt,omitempty"`
}

type Inventory struct {
	Crops map[string]int `json:"crops"`
	Seeds map[string]int `json:"seeds"`
}

type MarketState struct {
	AvailableSeeds []string           `json:"availableSeeds"`
	PriceModifiers map[string]float64 `json:"priceModifiers"`
	LastRotation   time.Time          `json:"lastRotation"`
}

type FarmState struct {
	Coins       int         `json:"coins"`
	Plots       []Plot      `json:"plots"`
	Inventory   Inventory   `json:"inventory"`
	Market      MarketState `json:"market"`
	LastUpdated time.Time   `json:"lastUpdated"`
}
