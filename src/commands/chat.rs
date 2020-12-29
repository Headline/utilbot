use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandError, CommandResult},
    model::prelude::*,
};

use crate::cache::MarkovCache;

#[command]
#[sub_commands(about)]
pub async fn chat(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let data = ctx.data.write().await;
    let markov = data.get::<MarkovCache>().unwrap().read().await;

    let chat = markov.get_string(msg.guild_id.unwrap());

    msg.channel_id.say(&ctx.http, chat).await?;
    debug!("Command executed");
    Ok(())
}

#[command]
async fn about(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let data = ctx.data.write().await;
    let markov = data.get::<MarkovCache>().unwrap().read().await;

    if args.is_empty() {
        return Err(CommandError::from(
            "You haven't given me something to chat about!",
        ));
    }

    let about = args.parse::<String>().unwrap();
    let chat = markov.get_string_from(msg.guild_id.unwrap(), &about);

    if chat.is_empty() {
        return Err(CommandError::from(
            "I haven't learned that word yet!",
        ));
    }
    msg.channel_id.say(&ctx.http, chat).await?;

    Ok(())
}