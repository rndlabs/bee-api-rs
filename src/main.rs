

use anyhow::{anyhow, Context, Result};
use clap::{Arg, Command};

#[tokio::main]
async fn main() -> Result<()> {
    let matches = clap::App::new("bee-rs")
        .about("bee api cli client")
        .arg(
            Arg::new("endpoint")
                .long("endpoint")
                .takes_value(true)
                .help("bee api endpoint"),
        )
        .subcommands(vec![
            Command::new("subscribe")
            .about("subscribe to a given topic")
            .arg(topic_flag()),
            Command::new("publish")
            .about("publish a message to a pss topic")
            .arg(topic_flag())
            .arg(recipient_flag())
            .arg(postage_batch_id_flag())
            .arg(target_flag())
            .arg(data_flag()),
            Command::new("addresses")
            .about("return node underlay and overlay addresses"),
        ])
        .get_matches();
    match matches.subcommand() {
        Some((("subscribe", s))) => {
            bee_api::pss::subscribe_topic(
                &matches.get_one::<String>("endpoint").unwrap(),
                &s.get_one::<String>("topic").unwrap(),
            )
            .await?;
        }
        Some(("publish", s)) => {
            bee_api::pss::publish_topic(
                reqwest::Client::new(),
                &matches.get_one::<String>("endpoint").unwrap(),
                &s.get_one::<String>("topic").unwrap(),
                &s.get_one::<String>("target-msg-prefix").unwrap(),
                &s.get_one::<String>("recipient").unwrap(),
                &s.get_one::<String>("postage-batch-id").unwrap(),
                s.get_one::<String>("data").unwrap().as_bytes()
            ).await?;
        }
        Some(("addresses", _)) => {
            match bee_api::get_addresses(
                &reqwest::Client::new(),
                &matches.get_one::<String>("endpoint").unwrap()
            ).await {
                Ok(res) => println!("{res:#?}"),
                Err(err) => return Err(anyhow!("failed to get addresses {err:#?}")),
            }
        }
        _ => return Err(anyhow!("invalid subcommand")),
    }
    Ok(())
}

fn topic_flag() -> Arg<'static> {
    Arg::new("topic")
        .long("topic")
        .takes_value(true)
        .help("topic to subscribe/publish too")
}

fn recipient_flag() -> Arg<'static> {
    Arg::new("recipient")
        .long("recipient")
        .takes_value(true)
        .help("recipient public key")
}

fn postage_batch_id_flag() -> Arg<'static> {
    Arg::new("postage-batch-id")
        .long("postage-batch-id")
        .alias("pbi")
        .takes_value(true)
        .help("postage stamp to use for storage")
}

fn target_flag() -> Arg<'static> {
    Arg::new("target-msg-prefix")
    .long("target-msg-prefix")
    .takes_value(true)
    .help("target message address prefix")
}

fn data_flag() -> Arg<'static> {
    Arg::new("data")
    .long("data")
    .takes_value(true)
    .help("arbitrary data to use with uploads")
}