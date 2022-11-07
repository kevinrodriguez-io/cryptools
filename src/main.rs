pub mod commands;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "cryptools", author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: ProgramCommand,
}

#[derive(Subcommand)]
enum ProgramCommand {
    Convert {
        #[arg(short, long)]
        amount: f64,
        #[arg(short, long)]
        input_currency: String,
        #[arg(short, long)]
        output_currency: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    match &cli.command {
        ProgramCommand::Convert {
            amount,
            input_currency,
            output_currency,
        } => {
            commands::convert(amount, input_currency, output_currency).await?;
        }
    }
    Ok(())
}
