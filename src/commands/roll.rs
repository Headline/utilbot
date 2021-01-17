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

use serenity::framework::standard::{macros::command, Args, CommandResult, CommandError};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::utls::discordhelpers;
use serenity::builder::CreateEmbed;
use crate::utls::constants::*;

extern crate rouler;

#[command]
pub async fn roll(ctx: &Context, msg: &Message, args: Args) -> CommandResult {

    let mut user_input = args.current();
    let num = args.parse::<i32>();

    // support for single number rolls (translation from |!roll 20| to |!roll 1d20|
    let str;
    if num.is_ok() {
        str = format!("1d{}", num.unwrap());
        user_input = Some(&str);
    }

    let mut roll_str;
    let roll;
    match user_input {
        Some(str) => {
            // if they didn't specify 1d20 - and instead just typed d20, lets handle it ourselves
            roll_str = String::from(str);
            if str.starts_with('d') {
                roll_str = format!("1{}", str);
            }
            roll = rouler::roll_dice_or_fail(&roll_str);
        },
        None => {    // default to 6 - but allow user input
            roll = rouler::roll_dice_or_fail("1d6");
        }
    }

    match roll {
        Ok(val) => {
            let mut emb = CreateEmbed::default();
            emb.thumbnail(ICON_ROLL);
            emb.footer(|f | {
                f.text(&format!("Requested by: {}", msg.author.tag()));
                f
            });
            emb.color(COLOR_OKAY);
            emb.description(format!("*Rolling...*\n\n**You've rolled: {}**", val));
            let mut emb_msg = discordhelpers::embed_message(emb);
            msg.channel_id
                .send_message(&ctx.http, |_| &mut emb_msg)
                .await?;
        },
        Err(e) => {
            return Err(CommandError::from(
                format!("Unable to parse input!\n```{}```\n\nRejected: '{}'", e, user_input.unwrap()),
            ));
        }
    }

    debug!("Command executed");
    Ok(())
}
