use std::time::SystemTime;

use crate::{
    market::generate_market,
    models::{FarmState, Inventory, Plot},
    persistence::save_farm,
};

pub fn init() {
    let farm = FarmState {
        coins: 150,
        plots: (0..3)
            .map(|_| Plot {
                id: uuid::Uuid::new_v4(),
                planted_crop: None,
                planted_at: None,
            })
            .collect::<Vec<_>>(),
        inventory: Inventory {
            crops: None,
            seeds: None,
        },
        market: generate_market(),
        last_updated: SystemTime::now(),
        has_seen_tutorial: false,
    };

    match save_farm(&farm) {
        true => println!("󰉉 termfarm save file initialised!"),
        false => {
            usefulog::err("failed to save farm");
            std::process::exit(1);
        }
    }
}
