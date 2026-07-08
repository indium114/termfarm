use crate::models::FarmState;
use dirs;
use std::fs;

pub fn save_path() -> String {
    let home = dirs::home_dir()
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_default();
    let dir = home + ".local/share/termfarm";

    _ = fs::create_dir_all(&dir);

    dir + "/save.json"
}

pub fn load_farm() -> FarmState {
    let path = save_path();
    let mut farm = fs::read_to_string(path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap();

    // TODO: update_market_if_needed(&farm)

    return farm;
}

pub fn save_farm(farm: &FarmState) -> bool {
    match serde_json::to_string_pretty(farm) {
        Ok(json) => fs::write(save_path(), json).is_ok(),
        Err(_) => false,
    }
}
