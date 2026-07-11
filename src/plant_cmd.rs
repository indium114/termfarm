use crate::persistence::save_farm;
use crate::{crops::crop_registry, persistence::load_farm};
use std::collections::HashMap;
use std::process::exit;
use std::time::SystemTime;

pub fn plant(seed_id: String) {
    let mut farm = load_farm();
    let registry = crop_registry();
    let now = SystemTime::now();

    let seeds = farm.inventory.seeds.get_or_insert_with(HashMap::new);
    let seed_count = seeds.entry(seed_id.clone()).or_insert(0);
    let index = farm
        .plots
        .iter()
        .position(|plot| plot.planted_crop.is_none());

    if !registry.contains_key(&seed_id.clone()) {
        usefulog::err(format!("Unknown seed: {seed_id}"));
        exit(1);
    }
    if *seed_count == 0 {
        println!("You don't have any {seed_id} seeds");
        exit(1);
    }

    match index {
        Some(index) => {
            farm.plots[index].planted_crop = Some(seed_id.clone());
            farm.plots[index].planted_at = Some(now);
            *seeds.entry(seed_id.clone()).or_insert(0) -= 1;
            if *seeds.entry(seed_id.clone()).or_insert(0) == 0 {
                seeds.remove_entry(&seed_id);
            }

            farm.last_updated = now;
            match save_farm(&farm) {
                true => {
                    let crop = &registry[&seed_id];
                    println!("󰜐 Planted {} {seed_id}", crop.icon);
                }
                false => {
                    usefulog::err("failed to save farm");
                    exit(1);
                }
            }
        }
        None => {
            println!(
                "No available plots.\nHarvest some crops or purchase more plots from the Market"
            );
            exit(1);
        }
    }
}
