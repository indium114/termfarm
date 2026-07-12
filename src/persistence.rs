use crate::{market::update_market_if_needed, models::FarmState};
use std::fs;

pub fn save_path() -> String {
    let home = dirs::home_dir()
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_default();

    #[cfg(target_os = "linux")]
    let dir = home + "/.local/share/termfarm";

    #[cfg(target_os = "macos")]
    let dir = home + "/.local/share/termfarm";

    #[cfg(target_os = "windows")]
    let dir = home + "\\AppData\\Roaming\\termfarm";

    _ = fs::create_dir_all(&dir);

    #[cfg(not(target_os = "windows"))]
    return dir + "/save.json";

    #[cfg(target_os = "windows")]
    return dir + "\\save.json";
}

pub fn load_farm() -> FarmState {
    let path = save_path();
    let mut farm = fs::read_to_string(path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap();

    update_market_if_needed(&mut farm);

    farm
}

pub fn save_farm(farm: &FarmState) -> bool {
    match serde_json::to_string_pretty(farm) {
        Ok(json) => fs::write(save_path(), json).is_ok(),
        Err(_) => false,
    }
}
