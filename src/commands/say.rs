use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::{send, MarkovChainer};

#[command]
pub async fn rsay(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let _ = msg.delete(ctx).await;

    let chain = {
        let data_read = ctx.data.read().await;
        data_read
            .get::<MarkovChainer>()
            .expect("Expected MessageCount in TypeMap.")
            .clone()
    };
    let chain = chain.read().await;

    let response = if args.is_empty() {
        chain.generate_str()
    } else {
        let token = args.single_quoted::<String>()?;
        chain.generate_str_from_token(token.as_str())
    };

    send!(&ctx, msg, response);
    Ok(())
}
