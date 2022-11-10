mod weather;
mod uptime;

use dotenv;
use reqwest;

use clap::{Parser, Subcommand};
use crate::uptime::{get_uptime_hours, get_uptime_minutes};

use crate::weather::{get_current_weather};



#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}


#[derive(Subcommand)]
enum Commands {
    /// Shows current weather information
    Weather { },
    /// Shows uptime. Arguments: hours, minutes
    Uptime { mode: String }
}


fn main() {
    dotenv::from_path("/home/dan/.config/gaf/.env").unwrap();

    let cli = Cli::parse();

    match &cli.command {
        Commands::Weather { } => println!("{}", get_current_weather()),
        Commands::Uptime { mode } => {
            if mode.as_str() == "hours" {
                println!("{}", get_uptime_hours())
            } else if mode.as_str() == "minutes" {
                println!("{}", get_uptime_minutes())
            } else {
                println!("The uptime command received unknown mode: {}", mode)
            }
        },
    }

}
