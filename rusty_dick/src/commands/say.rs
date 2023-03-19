use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[path = "../chain.rs"]
mod chain;
use crate::send;

#[command]
pub async fn say(ctx: &Context, msg: &Message, mut _args: Args) -> CommandResult {
    send!(&ctx, msg, chain::generate());
    Ok(())
}
