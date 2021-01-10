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

use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::utls::discordhelpers;
use serenity::builder::CreateEmbed;
use crate::utls::constants::*;

#[command]
pub async fn info(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    const AUTHOR: &'static str = env!("CARGO_PKG_AUTHORS");
    const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");


    let mut emb = CreateEmbed::default();
    emb.thumbnail(ICON_HELP);
    emb.footer(|f | {
        f.text(&format!("Requested by: {}", msg.author.tag()));
        f
    });
    emb.color(COLOR_OKAY);
    emb.description(format!("Hello! Thanks for looking into me!"));
    emb.field("Version", VERSION, false);
    emb.field("Description", DESCRIPTION, false);
    emb.field("Author", AUTHOR, false);
    emb.field("Language", "Rust 2018", false);
    emb.field("Repository", "[Click me](https://github.com/Headline/utilbot)", false);
    emb.field("Project License", "GNU Affero General Public License v3.0", false);
    let mut emb_msg = discordhelpers::embed_message(emb);
    msg.channel_id
        .send_message(&ctx.http, |_| &mut emb_msg)
        .await?;

    debug!("Command executed");
    Ok(())
}
