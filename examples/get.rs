use clap::{Parser, Subcommand};

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
    /// Returns sports
    Sports,
    /// Leagues in a particular sport
    Leagues {
        #[arg(short, long)]
        sport_id: i32,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let cli = Cli::parse();
    let client = pinnacle::Client::new(cli.pinnacle_username, cli.pinnacle_password);
    match cli.command {
        Command::Sports => {
            println!("{:#?}", client.get_sports().await?)
        }
        Command::Leagues { sport_id } => {
            println!("{:#?}", client.get_sport_leagues(sport_id).await?)
        }
    }
    Ok(())
}
