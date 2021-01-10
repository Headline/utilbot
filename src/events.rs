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
    async_trait,
    framework::{standard::macros::hook, standard::CommandResult},
    model::{
        channel::Message,
        event::ResumedEvent,
        gateway::Ready,
        guild::{Guild, GuildUnavailable},
    },
    prelude::*,
};

use crate::utls::discordhelpers;
use serenity::framework::standard::DispatchError;
use crate::cache::{MarkovCache, MarkovRegexCache, GuildCountCache};
use crate::markov::markovsaver;

pub struct Handler; // event handler for serenity


#[async_trait]
impl EventHandler for Handler {
    async fn guild_create(&self, ctx: Context, g: Guild) {
        let data = ctx.data.write().await;
        let mut markov = data.get::<MarkovCache>().unwrap().write().await;
        let gc = data.get::<GuildCountCache>().unwrap().read().await;

        debug!("Loading markov data for {}", g.name);
        markov.load_guild(g.id);

        if markov.chain_count() == gc.count {
            debug!("Starting markov save handler");
            markovsaver::start_listening(ctx.data.clone());
        }
    }

    async fn guild_delete(&self, ctx: Context, g: GuildUnavailable) {
        info!("Removing data for guild {}", g.id);
        let data = ctx.data.write().await;
        let mut markov = data.get::<MarkovCache>().unwrap().write().await;
        markov.delete_guild(g.id);
    }

    async fn message(&self, ctx: Context, msg: Message) {
        let data = ctx.data.read().await;
        let regex = data.get::<MarkovRegexCache>().unwrap().read().await;
        if !regex.is_match(&msg.content) {
            let mut markov = data.get::<MarkovCache>().unwrap().write().await;
            markov.add_message(msg.guild_id.unwrap(), &msg.content);
        }
    }

    async fn ready(&self, ctx: Context, rdy: Ready) {
        let data = ctx.data.read().await;
        let mut gc = data.get::<GuildCountCache>().unwrap().write().await;
        gc.count = rdy.guilds.len();
        info!("Bot Ready");
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}

#[hook]
pub async fn before(_: &Context, _: &Message, _: &str) -> bool {
    true
}

#[hook]
pub async fn after(ctx : &Context, msg: &Message, _: &str, command_result: CommandResult) {
    if let Err(e) = command_result {
        let emb = discordhelpers::build_fail_embed(&msg.author, &format!("{}", e));
        let mut emb_msg = discordhelpers::embed_message(emb);
        if msg
            .channel_id
            .send_message(&ctx.http, |_| &mut emb_msg)
            .await
            .is_err()
        {
            // missing permissions, just ignore...
        }
    }
}

#[hook]
pub async fn dispatch_error(ctx: &Context, msg: &Message, error: DispatchError) {
    if let DispatchError::Ratelimited(_) = error {
        let emb =
            discordhelpers::build_fail_embed(&msg.author, "You are sending requests too fast!");
        let mut emb_msg = discordhelpers::embed_message(emb);
        if msg
            .channel_id
            .send_message(&ctx.http, |_| &mut emb_msg)
            .await
            .is_err()
        {}
    }
}
