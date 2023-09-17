use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::send;

#[command]
pub async fn rroll(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let _ = msg.delete(ctx).await;
    let roll = d20::roll_dice(args.rest())?.to_string();

    send!(
        &ctx,
        msg,
        format!("{} rolled: {}", msg.author.mention(), roll)
    );

    Ok(())
}
