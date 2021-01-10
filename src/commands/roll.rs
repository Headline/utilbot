use serenity::framework::standard::{macros::command, Args, CommandResult, CommandError};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::utls::discordhelpers;
use serenity::builder::CreateEmbed;
use crate::utls::constants::*;
use rand::Rng;

#[command]
pub async fn roll(ctx: &Context, msg: &Message, args: Args) -> CommandResult {

    // default to 6 - but allow user input
    let mut high = 6;
    let user_input = args.parse::<i32>();
    if user_input.is_ok() {
        let num = user_input.unwrap();
        if num > 1 {
            high = num;
        }
    }
    else if !args.is_empty() {
        return Err(CommandError::from(
            format!("Unable to parse roll - input must be a non-negative integer.\n\n Rejected: `{}`", args.current().unwrap()),
        ));
    }

    let num : i32 = rand::thread_rng().gen_range(1..high);

    let mut emb = CreateEmbed::default();
    emb.thumbnail(ICON_ROLL);
    emb.footer(|f | {
        f.text(&format!("Requested by: {}", msg.author.tag()));
        f
    });
    emb.color(COLOR_OKAY);
    emb.description(format!("*Rolling...*\n\n**You've rolled: {}**", num));
    let mut emb_msg = discordhelpers::embed_message(emb);
    msg.channel_id
        .send_message(&ctx.http, |_| &mut emb_msg)
        .await?;

    debug!("Command executed");
    Ok(())
}
