use serde::Deserialize;
use serenity::all::GuildId;

use crate::Data;

#[derive(Clone, Debug, Deserialize)]
pub struct GuildConfig {
    pub prefix: Option<String>,
}

impl GuildConfig {
    pub fn new(prefix: Option<String>) -> Self {
        GuildConfig {
            prefix: prefix.or(Some("-".to_string())),
        }
    }

    #[inline]
    pub fn prefix(mut self, prefix: Option<String>) -> Self {
        self.prefix = prefix;
        self
    }
}

impl Default for GuildConfig {
    fn default() -> Self {
        Self::new(Some("-".to_string()))
    }
}

pub async fn add_guild_config_def(data: &Data, guild_id: GuildId) -> GuildConfig {
    let guild_cache = &data.guild_cache;
    let database = &data.database;

    let guild_config = GuildConfig::default();

    guild_cache.insert(guild_id, guild_config.clone());

    let _ = sqlx::query!(
        "INSERT INTO guilds (guild_id, prefix) VALUES ($1, $2) ON CONFLICT (guild_id) DO UPDATE SET prefix = $2",
        i64::from(guild_id),
        guild_config.prefix,
    )
    .execute(database)
    .await;

    guild_config
}

pub async fn cache_guild_config(data: &Data, guild_id: GuildId) -> GuildConfig {
    let guild_cache = &data.guild_cache;
    let database = &data.database;

    let config_sqlx = sqlx::query!(
        "SELECT * FROM guilds WHERE guild_id = $1",
        i64::from(guild_id)
    )
    .fetch_one(database)
    .await;

    // reset & apply/create new conf
    guild_cache.remove(&guild_id);
    match config_sqlx {
        Ok(record) => {
            let config = GuildConfig::new(record.prefix);
            guild_cache.insert(guild_id, config.clone());
            config
        }
        Err(err) => match err {
            sqlx::Error::RowNotFound => add_guild_config_def(data, guild_id).await,
            _ => {
                // Something is VERY wrong if this happens, SO it shouldn't happen
                // I will eventually implement a proper solution to this.
                GuildConfig::default()
            }
        },
    }
}

pub async fn get_guild_config(data: &Data, guild_id: GuildId) -> Option<GuildConfig> {
    let guild_cache = &data.guild_cache;

    let config_option = guild_cache.get(&guild_id);

    config_option.map(|c| c.value().clone())
}

pub async fn update_guild_config(data: &Data, guild_id: GuildId, new_config: GuildConfig) {
    let guild_cache = &data.guild_cache;
    let database = &data.database;

    guild_cache.remove(&guild_id);
    guild_cache.insert(guild_id, new_config.clone());

    let _ = sqlx::query!(
        "INSERT INTO guilds (guild_id, prefix) VALUES ($1, $2) ON CONFLICT (guild_id) DO UPDATE SET prefix = $2",
        i64::from(guild_id),
        new_config.prefix,
    )
    .execute(database)
    .await;
}
