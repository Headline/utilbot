// Utilbot - A random utility bot
// Copyright (C) 2020  Michael Flaherty
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandError, CommandResult},
    model::prelude::*,
};

use crate::cache::MarkovCache;

#[command]
#[sub_commands(about)]
pub async fn chat(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let data = ctx.data.read().await;
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