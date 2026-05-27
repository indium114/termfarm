package internal

import (
	"encoding/json"
	"os"
	"path/filepath"
)

func saveURL() string {
	home, _ := os.UserHomeDir()
	dir := filepath.Join(home, ".local", "share", "termfarm")

	os.MkdirAll(dir, 0755)

	return filepath.Join(dir, "save.json")
}

func loadFarm() (FarmState, error) {
	url := saveURL()
	data, err := os.ReadFile(url)
	if err != nil {
		return FarmState{}, err
	}

	var farm FarmState
	if err := json.Unmarshal(data, &farm); err != nil {
		return FarmState{}, err
	}

	updateMarketIfNeeded(&farm)

	return farm, nil
}

func saveFarm(farm FarmState) error {
	data, err := json.Marshal(farm)
	if err != nil {
		return err
	}

	tmpPath := saveURL() + ".tmp"
	err = os.WriteFile(tmpPath, data, 0644)
	if err != nil {
		return err
	}
	err = os.Rename(tmpPath, saveURL())
	if err != nil {
		return err
	}

	return nil
}
