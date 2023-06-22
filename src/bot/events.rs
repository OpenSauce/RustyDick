use serenity::model::prelude::{Activity, Message, Ready};
use serenity::{async_trait, prelude::*};

use crate::util::markov::MarkovChainer;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        ctx.set_activity(Activity::watching(msg.to_owned().author.name))
            .await;

        if !msg.author.bot && !msg.content.starts_with('.') {
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
