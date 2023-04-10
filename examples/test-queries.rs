//! Pinnacle Open API definition don't differentiate non optional respons fields.
//! This test loops through sports to confirm my assumption about optional fields.
use clap::Parser;
use pinnacle::prelude::*;

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

/// A helper to format error with its source chain
pub fn error_chain(e: &impl std::error::Error) -> String {
    use std::fmt::Write as _;

    let mut s = e.to_string();
    let mut current = e.source();
    if current.is_some() {
        s.push_str("\nCaused by:");
    }
    while let Some(cause) = current {
        write!(s, "\n\t{}", cause).ok();
        current = cause.source();
    }
    s
}
