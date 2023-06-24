use std::collections::HashSet;
use std::env;
use std::sync::Arc;

use markov::Chain;

use serenity::framework::standard::macros::group;
use serenity::framework::StandardFramework;
use serenity::http::Http;
use serenity::prelude::*;

use crate::bot::events::Handler;
use crate::commands::ping::*;
use crate::commands::say::*;
use crate::util::markov::{load_markov_chain, MarkovChainer};
use env_logger::Env;

#[group]
#[commands(ping, rsay)]
struct General;

pub async fn start() {
    env_logger::Builder::from_env(Env::default().default_filter_or("error")).init();
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

    let chain: Chain<String> = load_markov_chain();

    {
        let mut data = client.data.write().await;
        data.insert::<MarkovChainer>(Arc::new(RwLock::new(chain)))
    }

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
