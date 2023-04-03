use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(env, long)]
    pinnacle_username: String,

    #[arg(env, long)]
    pinnacle_password: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let cli = Cli::parse();
    let client = pinnacle::Client::new(cli.pinnacle_username, cli.pinnacle_password);
    let resp = client
        .get_straight_odds(&pinnacle::types::StraightOddsRequest {
            sport_id: 29,
            ..Default::default()
        })
        .await?;
    dbg!(resp);
    Ok(())
}
