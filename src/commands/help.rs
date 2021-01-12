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
    builder::CreateEmbed,
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};

use crate::utls::constants::*;
use crate::utls::discordhelpers;
use crate::cache::BotInfo;

#[command]
pub async fn help(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if !args.is_empty() {
        let prefix = {
            let data = ctx.data.read().await;
            let botinfo = data.get::<BotInfo>().unwrap().read().await;
            botinfo.get("BOT_PREFIX").unwrap().to_owned()
        };

        let cmd = args.message();
        let mut emb = CreateEmbed::default();
        emb.thumbnail(ICON_HELP);

        let unknown = format!("Unknown command '{}'", cmd);
        let description = match cmd {
            "help" => "Do you like recursion or something?",
            "roll" => {
                emb.title("Roll command");
                emb.field("Example", format!("{}roll", prefix), false);
                emb.field("Alternate Example", format!("{}roll 20", prefix), false);
                "Rolls a random number between 1 and 6 by default, if a number is specified then the result will be a number between 1 and said number."
            }
            "info" => {
                emb.title("Info command");
                emb.field("Example", format!("{}info", prefix), false);
                "Outputs some information regarding the bot's owner, project license, and GitHub repository."
            }
            "chat" => {
                emb.title("Chat command");
                emb.field("Example", format!("{}chat", prefix), false);
                "Outputs a chat response generated from the guild's chat messages.\nMessage generation is random (markov chain) and all chat data is stored on disk as graph form."
            }
            "repeat" => {
                emb.title("Repeat command");
                emb.field("Example", format!("{}repeat 5", prefix), false);
                "Repeats the last message you sent a number of times."
            }
            "osrs lookup" => {
                emb.title("OSRS Lookup command");
                emb.field("Example", format!("{}osrs lookup <playername>", prefix), false);
                "Looks up a player's stats on Jagex's Old School Runescape's high scores"
            }
            _ => {
                emb.title("Command not found");
                emb.color(COLOR_FAIL);
                emb.thumbnail(ICON_FAIL);
                unknown.as_str()
            }
        };

        emb.description(description);

        let mut emb_msg = discordhelpers::embed_message(emb);
        msg.channel_id
            .send_message(&ctx.http, |_| &mut emb_msg)
            .await?;

        return Ok(());
    }

    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.thumbnail(ICON_HELP);
            e.description("Here's all my commands!\n\nIf you need help with a particular command type ;help followed by the command name.");
            e.color(COLOR_OKAY);
            e.title("Commands");
            e.field("roll", "``` Rolls a playing die ```", false);
            e.field("chat", "``` Generates a random message based on chat history ```", false);
            e.field("osrs lookup", "``` Looks up a player's stats on Jagex's Old School Runescape's high scores ```", false);
            e.field("info", "``` Outputs some information regarding the bot's owner, project license, and GitHub repository ```", false);
            e.field("repeat", "``` Repeats the last message you sent a number of times ```", false);
            e
        })
    }).await?;

    debug!("Command executed");
    Ok(())
}
