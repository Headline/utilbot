use serenity::prelude::{TypeMap, TypeMapKey};
use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::RwLock;
use serenity::model::id::UserId;
use std::error::Error;
use crate::markov::markov::MarkovManager;
use regex::Regex;

/** Caching **/
pub struct BotInfo;
impl TypeMapKey for BotInfo {
    type Value = Arc<RwLock<HashMap<&'static str, String>>>;
}

pub struct MarkovCache;
impl TypeMapKey for MarkovCache {
    type Value = Arc<RwLock<MarkovManager>>;
}

pub struct GuildCountCache;
pub struct GuildCount {
    pub count : usize
}
impl TypeMapKey for GuildCountCache {
    type Value = Arc<RwLock<GuildCount>>;
}

pub struct MarkovRegexCache;
impl TypeMapKey for MarkovRegexCache {
    type Value = Arc<RwLock<Regex>>;
}
pub async fn fill(
    data: Arc<RwLock<TypeMap>>,
    prefix: &str,
    id: &UserId,
) -> Result<(), Box<dyn Error>> {
    let mut data = data.write().await;

    // Lets map some common things in BotInfo
    let mut map = HashMap::<&str, String>::new();
    map.insert("BOT_PREFIX", String::from(prefix));
    map.insert("BOT_ID", id.to_string());
    data.insert::<BotInfo>(Arc::new(RwLock::new(map)));

    data.insert::<MarkovCache>(Arc::new(RwLock::new(MarkovManager::new())));

    data.insert::<MarkovRegexCache>(Arc::new(RwLock::new(Regex::new(r"(\?|!|/|-|\+|@|#|\$|%|\^|&|\*|\.)[A-Za-z0-9_.]").unwrap())));
    data.insert::<GuildCountCache>(Arc::new(RwLock::new(GuildCount{count:0})));
    Ok(())
}
