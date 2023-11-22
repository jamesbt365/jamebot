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

    pub fn _get_prefix(&self) -> Option<&String> {
        self.prefix.as_ref()
    }
}

impl Default for GuildConfig {
    fn default() -> Self {
        Self::new(Some("-".to_string()))
    }
}

pub async fn add_guild_config_def(data: &Data, guild_id: GuildId) -> Option<GuildConfig> {
    let guild_cache = &data.guild_cache;
    let database = &data.database;

    let guild_config = GuildConfig::default();

    let config = guild_cache.insert(guild_id, guild_config.clone());

    if config.is_some() {
        let _ = sqlx::query!(
            "INSERT INTO guilds (guild_id, prefix) VALUES ($1, $2) ON CONFLICT (guild_id) DO UPDATE SET prefix = $2",
            i64::from(guild_id),
            guild_config.prefix,
        )
        .execute(database)
        .await;
    };

    config
}
