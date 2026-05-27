package internal

import (
	"math/rand"
	"time"
)

const marketRotationInterval = 4 * 60 * 60 * time.Second
const marketMaxItems = 3

func randomInRange(lo, hi float64) float64 {
	return lo + rand.Float64()*(hi-lo)
}

func generateMarket() MarketState {
	keys := make([]string, 0, len(cropRegistry))
	for k := range cropRegistry {
		keys = append(keys, k)
	}
	rand.Shuffle(len(keys), func(i, j int) { keys[i], keys[j] = keys[j], keys[i] })
	n := marketMaxItems
	if n > len(keys) {
		n = len(keys)
	}
	selection := keys[:n]
	modifiers := make(map[string]float64)
	for _, seed := range selection {
		modifiers[seed] = randomInRange(0.7, 1.3)
	}
	return MarketState{
		AvailableSeeds: selection,
		PriceModifiers: modifiers,
		LastRotation:   time.Now(),
	}
}

func updateMarketIfNeeded(farm *FarmState) {
	now := time.Now()
	if now.Sub(farm.Market.LastRotation) < marketRotationInterval {
		return
	}
	keys := make([]string, 0, len(cropRegistry))
	for k := range cropRegistry {
		keys = append(keys, k)
	}
	rand.Shuffle(len(keys), func(i, j int) { keys[i], keys[j] = keys[j], keys[i] })
	n := marketMaxItems
	if n > len(keys) {
		n = len(keys)
	}
	selection := keys[:n]
	modifiers := make(map[string]float64)
	for _, seed := range selection {
		modifiers[seed] = randomInRange(0.7, 1.3)
	}
	farm.Market = MarketState{
		AvailableSeeds: selection,
		PriceModifiers: modifiers,
		LastRotation:   now,
	}
}

func buyPrice(cropID string, farm FarmState) (int, bool) {
	crop, ok := cropRegistry[cropID]
	if !ok {
		return 0, false
	}
	modifier, ok := farm.Market.PriceModifiers[cropID]
	if !ok {
		return 0, false
	}
	return int(float64(crop.BaseBuyPrice) * modifier), true
}

func sellPrice(cropID string, farm FarmState) int {
	crop := cropRegistry[cropID]
	modifier := 1.0
	if m, ok := farm.Market.PriceModifiers[cropID]; ok {
		modifier = m
	}
	return int(float64(crop.BaseSellPrice) * modifier)
}
