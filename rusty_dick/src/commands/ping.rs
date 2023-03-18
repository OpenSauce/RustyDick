use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::send;

#[command]
pub async fn ping(ctx: &Context, msg: &Message, mut _args: Args) -> CommandResult {
    send!(&ctx, msg, "Pong!");

    Ok(())
}
