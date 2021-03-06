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

use std::collections::HashMap;
use markov::Chain;
use serenity::model::id::GuildId;

pub struct MarkovManager {
    chains : HashMap<GuildId, Chain<String>>
}

impl MarkovManager {
    pub fn new() -> MarkovManager {
        MarkovManager {
            chains : HashMap::new()
        }
    }

    pub fn chain_count(&self) -> usize {self.chains.len()}

    pub fn save_all(&self) {
        for (k, v) in &self.chains {
            v.save(format!("markov/{}_markov", k)).unwrap();
        }
    }

    pub fn get_string_from(&self, guild : GuildId, token : &str) -> String {
        let chain = self.chains.get(&guild);
        match chain {
            Some(c) => {
                c.generate_str_from_token(&token)
            },
            None => String::new()
        }
    }

    pub fn delete_guild(&mut self, guild : GuildId) {
        self.chains.remove(&guild);
        std::fs::remove_file(format!("markov/{}_markov", guild))
            .unwrap_or_else(|_| panic!("Unable to clean data for guild {}", guild));
    }

    pub fn load_guild(&mut self, guild : GuildId) {
        let chain = Chain::load(format!("markov/{}_markov", guild));
        match chain {
            Ok(c) => {
                self.chains.insert(guild, c);
            }
            Err(_) => {
                debug!("New guild joined, adding new markov chain.");
                self.chains.insert(guild, Chain::new());
            }
        }
    }

    pub fn get_string(&self, guild : GuildId) -> String {
        let chain = self.chains.get(&guild);
        match chain {
            Some(c) => {
                c.generate_str()
            },
            None => String::new()
        }
    }

    pub fn add_message(&mut self, guild : GuildId, message : &str) {
        let chain = self.chains.get_mut(&guild);
        match chain {
            Some(c) => {
                c.feed_str(message);
            }
            None => {
                let mut chain = Chain::new();
                chain.feed_str(message);
                self.chains.insert(guild, chain);
            }
        }
    }
/*
    fn save_guild(&self, guild : GuildId) -> bool {
        let chain = self.chains.get(&guild);
        match chain {
            Some(c) => {
                c.save(format!("markov/{}_markov", guild)).is_ok()
            }
            None => {
                false
            }
        }
    }
*/
}