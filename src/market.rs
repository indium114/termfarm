use crate::crops::crop_registry;
use crate::models::MarketState;
use rand::RngExt;
use rand::seq::SliceRandom;
use std::collections::HashMap;
use std::ops::RangeInclusive;
use std::time::SystemTime;

static MARKET_ROTATION_INTERVAL: f64 = (4 * 60 * 60) as f64;
static MARKET_MAX_ITEMS: usize = 3;
static PRICE_MODIFIER_RANGE: RangeInclusive<f64> = 0.7..=1.3;

pub fn generate_market() -> MarketState {
    let registry = crop_registry();
    let mut rng = rand::rng();

    let mut all_seeds: Vec<&String> = registry.keys().collect();
    all_seeds.shuffle(&mut rng);
    let selection: Vec<String> = all_seeds
        .into_iter()
        .cloned()
        .take(MARKET_MAX_ITEMS)
        .collect();

    let mut modifiers: HashMap<String, f64> = HashMap::new();
    for seed in &selection {
        modifiers.insert(seed.to_string(), rng.random_range(PRICE_MODIFIER_RANGE.clone()));
    }

    MarketState {
        available_seeds: selection.iter().cloned().collect(),
        price_modifiers: modifiers,
        last_rotation: SystemTime::now(),
    }
}
