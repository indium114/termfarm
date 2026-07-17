use serde::{Deserialize, Serialize};
use std::{collections::HashMap, time::SystemTime};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct CropType {
    pub icon: &'static str,
    pub id: &'static str,
    pub grow_time: u16,
    pub base_buy_price: u16,
    pub base_sell_price: u16,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Plot {
    pub id: Uuid,
    pub planted_crop: Option<String>,
    pub planted_at: Option<SystemTime>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Inventory {
    pub crops: Option<HashMap<String, u16>>,
    pub seeds: Option<HashMap<String, u16>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MarketState {
    pub available_seeds: Vec<String>,
    pub price_modifiers: HashMap<String, f64>,
    pub last_rotation: SystemTime,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FarmState {
    pub coins: u32,
    pub plots: Vec<Plot>,
    pub inventory: Inventory,
    pub market: MarketState,
    pub last_updated: SystemTime,
    #[serde(default)]
    pub has_seen_tutorial: bool,
}
