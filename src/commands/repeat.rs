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

use crate::cache::LastMessageCache;
use string_builder::Builder;

#[command]
pub async fn repeat(ctx: &Context, msg: &Message, args: Args) -> CommandResult {

    let user_input = args.parse::<i32>();
    if !user_input.is_ok() {
        return Err(CommandError::from(
            format!("Unable to parse repeat count - input must be a non-negative integer > 0.\n\n Rejected: `{}`", args.current().unwrap()),
        ));
    }

    let mut repeat_count = user_input.unwrap();
    if repeat_count < 1 {
        return Err(CommandError::from(
            format!("Unable to parse repeat count - input must be a non-negative integer > 0.\n\n Rejected: `{}`", args.current().unwrap()),
        ));
    }

    let data = ctx.data.read().await;
    let mut message_cache = data.get::<LastMessageCache>().unwrap().lock().await;

    match message_cache.get_mut(&msg.author.id) {
        Some(m) => {
            const MAX_MESSAGE_LEN: usize = 1000;
            let mut builder = Builder::new(MAX_MESSAGE_LEN);
            while repeat_count > 0 {
                // limit ourselves - stop appending once we hit it.
                if builder.len() + m.len() > MAX_MESSAGE_LEN {
                    break;
                }
                builder.append(format!("{}\n", m));
                repeat_count -= 1;
            }
            msg.channel_id.say(&ctx.http, builder.string().unwrap()).await?;
        }
        None => {
            return Err(CommandError::from(
                "Unable to find your last message - ensure it was sent recently! I can be forgetful sometimes..."
            ));
        }
    }

    debug!("Command executed");
    Ok(())
}
