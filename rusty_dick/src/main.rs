mod commands;

use std::collections::HashSet;
use std::env;
use std::sync::Arc;

use serenity::async_trait;
use serenity::framework::standard::macros::group;
use serenity::framework::StandardFramework;
use serenity::http::Http;
use serenity::model::channel::Message;
use serenity::model::gateway::Activity;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use tokio::sync::RwLock;

use crate::commands::chatgpt::*;
use crate::commands::ping::*;
use crate::commands::say::*;

use markov::Chain;

struct MarkovChainer;

impl TypeMapKey for MarkovChainer {
    type Value = Arc<RwLock<Chain<String>>>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        ctx.set_activity(Activity::watching(msg.to_owned().author.name))
            .await;

        if !msg.author.bot && !msg.content.starts_with(".") {
            let chain = {
                let data_read = ctx.data.write().await;
                data_read
                    .get::<MarkovChainer>()
                    .expect("Expected Markov Chain in TypeMap.")
                    .clone()
            };
            let mut chain = chain.write().await;
            chain.feed_str(msg.content.as_str());
        }
    }

    async fn ready(&self, ctx: Context, _ready: Ready) {
        ctx.set_activity(Activity::watching("Rusty Anime")).await;
    }
}

#[group]
#[commands(ping, chatgpt, rsay)]
struct General;

#[tokio::main]
async fn main() {
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
        .configure(|c| c.prefix("."))
        .group(&GENERAL_GROUP);

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<MarkovChainer>(Arc::new(RwLock::new(Chain::new())))
    }

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
