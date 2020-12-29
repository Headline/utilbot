use std::sync::Arc;
use serenity::prelude::TypeMap;
use tokio::sync::RwLock;
use crate::cache::MarkovCache;
use std::thread::sleep;

extern crate time;
use time::PreciseTime;

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