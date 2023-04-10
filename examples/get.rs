use clap::{Parser, Subcommand};
use pinnacle::prelude::*;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(env, long)]
    pinnacle_username: String,

    #[arg(env, long)]
    pinnacle_password: String,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Returns current balance
    Balance,
    /// Returns sports
    Sports,
    /// Leagues in a particular sport
    Leagues { sport_id: i32 },
    /// Periods in a particular sport
    Periods { sport_id: i32 },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use Command::*;

    dotenvy::dotenv().ok();
    let cli = Cli::parse();
    let client = PinnacleClient::new(cli.pinnacle_username, cli.pinnacle_password);
    match cli.command {
        Balance => {
            println!("{:#?}", client.get(&GetClientBalance).await?)
        }
        Sports => {
            println!("{:#?}", client.get(&GetSports).await?)
        }
        Leagues { sport_id } => {
            println!("{:#?}", client.get(&GetLeagues { sport_id }).await?)
        }
        Periods { sport_id } => {
            println!("{:#?}", client.get(&GetPeriods { sport_id }).await?)
        }
    }
    Ok(())
}
