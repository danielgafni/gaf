mod calendar;
mod uptime;
mod weather;

use dotenv;

use crate::calendar::get_calendar_info;
use crate::uptime::{get_uptime_hours, get_uptime_minutes};
use crate::weather::get_current_weather;
use clap::{Parser, Subcommand};

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
    Weather {},
    /// Shows uptime. Arguments: hours, minutes
    Uptime { mode: String },
    /// Shows current date, weekday, hour and minute
    Calendar {},
}

fn main() {
    dotenv::from_path("/home/dan/.config/gaf/.env").unwrap();

    let cli = Cli::parse();

    match &cli.command {
        Commands::Weather {} => println!("{}", get_current_weather()),
        Commands::Uptime { mode } => {
            if mode.as_str() == "hours" {
                println!("{}", get_uptime_hours())
            } else if mode.as_str() == "minutes" {
                println!("{}", get_uptime_minutes())
            } else {
                println!("The uptime command received unknown mode: {}", mode)
            }
        }
        Commands::Calendar {} => println!("{}", get_calendar_info()),
    }
}
