use serde::{Deserialize, Serialize};
use std::{collections::HashMap, time::SystemTime};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct CropType {
    pub icon: String,
    pub id: String,
    pub grow_time: String,
    pub base_buy_price: u16,
    pub base_sell_price: u16,
}

#[derive(Deserialize, Serialize)]
pub struct Plot {
    pub uuid: Uuid,
    pub planted_crop: Option<String>,
    pub planted_at: Option<SystemTime>,
}

#[derive(Deserialize, Serialize, Default)]
pub struct Inventory {
    pub crops: HashMap<String, u16>,
    pub seeds: HashMap<String, u16>,
}

#[derive(Deserialize, Serialize)]
pub struct MarketState {
    pub available_seeds: Vec<String>,
    pub price_modifiers: HashMap<String, f64>,
    pub last_rotation: SystemTime,
}

#[derive(Deserialize, Serialize)]
pub struct FarmState {
    pub coins: u32,
    pub plots: Vec<Plot>,
    pub inventory: Inventory,
    pub market: MarketState,
    pub last_updated: SystemTime,
}
