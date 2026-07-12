use std::process::exit;

use clap::{Parser, Subcommand};

mod crops;
mod market;
mod models;
mod persistence;
mod plot_pricing;
mod sell;
mod stats;
mod tui;

mod buy_cmd;
mod buyplot_cmd;
mod harvest_cmd;
mod init_cmd;
mod inventory_cmd;
mod market_cmd;
mod plant_cmd;
mod sell_cmd;
mod stats_cmd;
mod view_cmd;

static VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[command(name = "termfarm")]
#[command(version = &VERSION)]
#[command(about = "A terminal idle farming game", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialise a new termfarm save file
    Init,
    /// Show farm statistics
    Stats,
    /// View the seed market
    Market,
    /// Buy seeds from the market
    Buy { seed_id: String, amount: u16 },
    /// Plant a seed
    Plant { seed_id: String },
    /// Harvest all mature crops
    Harvest,
    /// List the contents of your inventory
    Inventory,
    /// Sell crops from your inventory
    Sell {
        crop_id: String,
        amount: Option<u16>,
    },
    /// Buy a new plot
    BuyPlot {
        #[arg(long)]
        showprice: bool,
    },
    /// View the status of farm plots
    View,
}

fn main() {
    if std::env::args().count() == 1 {
        tui::run();
        exit(0)
    }

    let cli = Cli::parse();

    match &cli.command {
        Commands::Init => {
            init_cmd::init();
        }
        Commands::Stats => {
            stats_cmd::stats();
        }
        Commands::Market => {
            market_cmd::market();
        }
        Commands::Buy { seed_id, amount } => {
            buy_cmd::buy(seed_id.to_string(), *amount, true);
        }
        Commands::Plant { seed_id } => {
            plant_cmd::plant(seed_id.to_string(), true);
        }
        Commands::Harvest => {
            harvest_cmd::harvest(true);
        }
        Commands::Inventory => {
            inventory_cmd::inventory();
        }
        Commands::Sell { crop_id, amount } => {
            sell_cmd::sell(crop_id.to_string(), amount.unwrap_or(0));
        }
        Commands::BuyPlot { showprice } => {
            buyplot_cmd::buyplot(*showprice);
        }
        Commands::View => {
            view_cmd::view();
        }
    }
}
