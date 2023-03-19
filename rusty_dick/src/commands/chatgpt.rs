use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::env;

use crate::send;

#[derive(Debug, Serialize, Deserialize)]
struct ChatGPTRequest {
    query: String,
}

#[command]
pub async fn chatgpt(ctx: &Context, msg: &Message, mut _args: Args) -> CommandResult {
    msg.react(&ctx, '🔎').await.unwrap();
    let query = msg.content.split_at(9).1;
    match call_chatgpt(query).await {
        Ok(response) => {
            if response.len() >= 2000 {
                let response = response.split_at(1999);
                send!(&ctx, msg, response.0);
                send!(&ctx, msg, response.1);
            } else {
                send!(&ctx, msg, response);
            }
            msg.react(&ctx, '✅').await.unwrap();
        }
        Err(e) => {
            send!(&ctx, msg, e);
            msg.react(&ctx, '❌').await.unwrap();
        }
    };

    Ok(())
}

async fn call_chatgpt(query: &str) -> Result<String> {
    let new_query = ChatGPTRequest {
        query: query.to_owned(),
    };

    match reqwest::Client::new()
        .post(env::var("CHATGPT_URL").expect("No CHATGPT_URL"))
        .json(&new_query)
        .send()
        .await
    {
        Ok(resp) => match resp.status() {
            reqwest::StatusCode::OK => Ok(resp.text().await.unwrap()),
            reqwest::StatusCode::UNAUTHORIZED => Err(anyhow!("Unauthorized, refresh token?")),
            _ => Err(anyhow!("An error has occurred")),
        },
        Err(_) => Err(anyhow!("Unable to contact ChatGPT server.")),
    }
}
