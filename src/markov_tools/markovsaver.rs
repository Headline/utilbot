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

use std::sync::Arc;
use serenity::prelude::TypeMap;
use tokio::sync::RwLock;
use crate::cache::MarkovCache;
use std::thread::sleep;

pub fn start_listening(data: Arc<RwLock<TypeMap>>) {
    tokio::spawn(async move {
        let data = data.clone();
        loop {
            sleep(core::time::Duration::new(600, 0));
            let data = data.read().await;
            let markov = data.get::<MarkovCache>().unwrap().read().await;
            markov.save_all();
        }
    });
}