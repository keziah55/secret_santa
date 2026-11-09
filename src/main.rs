use std::path::PathBuf;
use clap::Parser;

#[derive(Parser)]
#[command(version = "1.0")]
#[command(about="Generate Secret Santa pairings and send emails", long_about=None)]
pub struct Cli {
    /// Path to dir of csv files defining people
    data_path: String,

    /// Update `history.csv` file
    #[arg(short = 'u', long)]
    update: bool,
}

fn main() {
    let cli = Cli::parse();
    let data_path = PathBuf::from(cli.data_path);
    secret_santa::generate(&data_path, cli.update);
}
