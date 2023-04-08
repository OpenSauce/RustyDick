use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::{send, MarkovChainer};

#[command]
pub async fn rsay(ctx: &Context, msg: &Message, mut _args: Args) -> CommandResult {
    let chain = {
        let data_read = ctx.data.read().await;
        data_read
            .get::<MarkovChainer>()
            .expect("Expected MessageCount in TypeMap.")
            .clone()
    };
    let chain = chain.read().await;
    send!(&ctx, msg, chain.generate_str());
    Ok(())
}
