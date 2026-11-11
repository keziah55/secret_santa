use clap::Parser;
use std::path::PathBuf;


mod email;

#[derive(Parser)]
#[command(version = "1.0")]
#[command(about="Generate Secret Santa pairings and send emails", long_about=None)]
pub struct Cli {
    /// Path to dir of csv files defining people
    data_path: String,

    /// Update `history.csv` file
    #[arg(short = 'u', long)]
    update: bool,

    /// Exclude `N` recent years from potential matches
    #[arg(long, short = 's', value_name = "N")]
    history_size: Option<usize>,
}

fn main() {
    let cli = Cli::parse();
    let data_path = PathBuf::from(cli.data_path);
    let history_size: usize  = match cli.history_size {
        Some(size) => size,
        None => 0,
    };
    secret_santa::generate(&data_path, history_size, cli.update);
}
