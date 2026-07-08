use clap::{Parser, Subcommand};

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
    Init,
    Stats,
    Market,
    Buy { seed_id: String, amount: u32 },
    Plant { seed_id: String },
    Harvest,
    Inventory,
    Sell { seed_id: String, amount: u32 },
    BuyPlot,
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
