use std::time::Duration;

use colorize::AnsiColor;
use humantime::format_duration;

use crate::{persistence::load_farm, stats::compute_stats};

pub fn stats() {
    let farm = load_farm();
    let stats = compute_stats(&farm);

    let trend_icon = if stats.market_trend > 0.0 {
        "󰔵"
    } else if stats.market_trend < 0.0 {
        "󰔳"
    } else {
        "󰔴"
    };

    let trend_pct = format!("{:.1}%%", stats.market_trend * 100.0);
    let total_planted = stats.ready_to_harvest + stats.growing_crops;
    let next_rotation =
        format_duration(Duration::from_secs(stats.next_market_rotation_in.as_secs()));

    let ready = format!(
        " {}/{}/{}",
        stats.ready_to_harvest, total_planted, stats.total_plots
    )
    .green();
    let inventory_crops = format!("󰜦 {} crops in inventory", stats.inventory_crops).blue();
    let inventory_seeds = format!("󰹢 {} seeds in inventory", stats.inventory_seeds).cyan();
    let wallet = format!(" {} coins in inventory", stats.coins).yellow();
    let trend = format!("{trend_icon} {trend_pct}").red();
    let rotate = format!("  {next_rotation} until next market rotation").magenta();

    println!("{ready} | {inventory_crops} | {inventory_seeds} | {wallet} | {trend} | {rotate}")
}
