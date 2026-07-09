use crate::{crops::crop_registry, persistence::load_farm};

pub fn inventory() {
    let farm = load_farm();
    let registry = crop_registry();

    println!("󰜦 Inventory:");
    println!("------------");
    println!(" Coins: {}", farm.coins);
    println!("󰹢 Seeds:");

    match farm.inventory.seeds {
        Some(seeds) => {
            if seeds.is_empty() {
                println!(" none");
            }
            for (seed, amount) in seeds {
                let icon = registry[&seed].icon;
                println!(" - {icon} {amount}x {seed}");
            }
        }
        None => println!(" none"),
    }

    println!();
    println!(" Crops:");

    match farm.inventory.crops {
        Some(crops) => {
            if crops.is_empty() {
                println!(" none");
            }
            for (crop, amount) in crops {
                let icon = registry[&crop].icon;
                println!(" - {icon} {amount}x {crop}");
            }
        }
        None => println!(" none"),
    }
}
