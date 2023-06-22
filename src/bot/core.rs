use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::sync::Arc;
use std::time::Instant;

use markov::Chain;

use serenity::framework::standard::macros::group;
use serenity::framework::StandardFramework;
use serenity::http::Http;
use serenity::prelude::*;
use tokio::sync::RwLock;

use crate::bot::events::Handler;
use crate::commands::ping::*;
use crate::commands::say::*;
use crate::util::markov::MarkovChainer;

#[group]
#[commands(ping, rsay)]
struct General;

pub async fn start() {
    dotenv::dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("No DISCORD_TOKEN environment variable");
    let http = Http::new(&token);

    let (_owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        }
        Err(e) => panic!("Could not retrieve bot information {e}"),
    };

    let framework = StandardFramework::new()
        .configure(|config: &mut serenity::framework::standard::Configuration| config.prefix("."))
        .group(&GENERAL_GROUP);

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    let mut chain: Chain<String> = Chain::new();

    let start = Instant::now();
    let lines = read_lines("./history/log.txt".to_string());
    for line in lines {
        let line = line.unwrap();
        chain.feed_str(line.as_str());
    }

    let duration = start.elapsed();

    println!("Log load time is: {:?}", duration);

    {
        let mut data = client.data.write().await;
        data.insert::<MarkovChainer>(Arc::new(RwLock::new(chain)))
    }

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}

fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    // Open the file in read-only mode.
    let file = File::open(filename).unwrap();
    // Read the file line by line, and return an iterator of the lines of the file.
    io::BufReader::new(file).lines()
}
