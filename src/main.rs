use clap::{Parser, Subcommand};

mod models;
mod persistence;
mod plot_pricing;

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
    Sell { seed_id: String, amount: u16 },
    /// Buy a new plot
    BuyPlot,
    /// View the status of farm plots
    View,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init => {
            todo!()
        }
        Commands::Stats => {
            todo!()
        }
        Commands::Market => {
            todo!()
        }
        Commands::Buy { seed_id, amount } => {
            todo!()
        }
        Commands::Plant { seed_id } => {
            todo!()
        }
        Commands::Harvest => {
            todo!()
        }
        Commands::Inventory => {
            todo!()
        }
        Commands::Sell { seed_id, amount } => {
            todo!()
        }
        Commands::BuyPlot => {
            todo!()
        }
        Commands::View => {
            todo!()
        }
    }
}
