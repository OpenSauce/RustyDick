use std::collections::HashSet;
use std::env;

use dotenv;
use serde::{Deserialize, Serialize};
use serenity::async_trait;
use serenity::http::Http;
use serenity::model::channel::Message;
use serenity::model::gateway::Activity;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
struct ChatGPTRequest {
    query: String,
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        ctx.set_activity(Activity::watching(msg.to_owned().author.name))
            .await;

        if !msg.content.starts_with(".") && !msg.content.starts_with("!") {
            return;
        }

        if msg.content.starts_with(".chatgpt") {
            msg.react(&ctx, '🔎').await.unwrap();
            let query = msg.content.split_at(9).1;
            match call_chatgpt(query).await {
                Ok(v) => {
                    if v.len() >= 2000 {
                        let v = v.split_at(1999);
                        if let Err(why) = msg.reply(&ctx, v.0).await {
                            println!("Error getting chatGPT response: {:?}", why);
                        }
                        if let Err(why) = msg.reply(&ctx, v.1).await {
                            println!("Error getting chatGPT response: {:?}", why);
                        }
                    } else {
                        if let Err(why) = msg.reply(&ctx, v).await {
                            println!("Error getting chatGPT response: {:?}", why);
                        }
                    }
                    msg.react(&ctx, '✅').await.unwrap();
                }
                Err(e) => {
                    if let Err(why) = msg.channel_id.say(&ctx, e).await {
                        println!("Error getting chatGPT response: {:?}", why);
                    }
                    msg.react(&ctx, '❌').await.unwrap();
                }
            };

            return;
        }

        match msg.content.as_str() {
            "!ping" => {
                if let Err(why) = msg.channel_id.say(ctx, "Pong!").await {
                    println!("Error sending message: {:?}", why);
                }
            }
            _ => println!("Command not found"),
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        ctx.set_activity(Activity::watching("Rusty Anime")).await;
    }
}

async fn call_chatgpt(query: &str) -> Result<String, &str> {
    let new_query = ChatGPTRequest {
        query: query.to_owned(),
    };
    match reqwest::Client::new()
        .post("http://localhost:3000/")
        .json(&new_query)
        .send()
        .await
    {
        Ok(resp) => match resp.status() {
            reqwest::StatusCode::OK => return Ok(resp.text().await.unwrap()),
            reqwest::StatusCode::UNAUTHORIZED => return Err("Unauthorized, refresh token?"),
            _ => return Err("An error has occurred."),
        },
        Err(_) => return Err("Unable to contact ChatGPT server."),
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let key = "DISCORD_TOKEN";

    let token;
    match dotenv::var(key) {
        Ok(v) => {
            token = v;
            println!("Hi {}", token)
        }
        Err(_e) => {
            token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
        }
    };

    let http = Http::new(&token);

    let (_owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        }
        Err(e) => panic!("Could not retrieve bot information {}", e),
    };

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
