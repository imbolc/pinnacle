//! Pinnacle Open API definition don't differentiate non optional response fields.
//! This test loops through sports to confirm my assumption about optional fields.
use clap::Parser;
use pinnacle::prelude::*;
use pinnacle::util::error_chain;

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
    let client = PinnacleClient::new(cli.pinnacle_username, cli.pinnacle_password);

    let sports: Vec<_> = client
        .get(&GetSports)
        .await?
        .sports
        .into_iter()
        .filter(|sport| sport.has_offerings)
        .filter(|sport| sport.event_count > 0)
        .collect();

    for (i, sport) in sports.iter().enumerate() {
        println!("{i} of {}", sports.len());
        let resp = client
            .get(&GetStraightOdds {
                sport_id: sport.id,
                ..Default::default()
            })
            .await;
        match resp {
            Ok(_) => (),
            Err(e) => eprintln!("{sport:?}, Error: {}\n", error_chain(&e)),
        }
    }
    Ok(())
}
