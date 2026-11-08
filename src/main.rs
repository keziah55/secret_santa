use clap::Parser;

#[derive(Parser)]
#[command(version = "1.0")]
#[command(about="Generate Secret Santa pairings and send emails", long_about=None)]
pub struct Cli {
    /// Path to dir of csv files defining people
    data_path: String,
}

fn main() {
    let cli = secret_santa::Cli::parse();
    let data_path = PathBuf::from(cli.data_path);
    secret_santa::generate(data_path);
}
