use std::sync::Arc;

use dashmap::DashMap;
use serenity::all::GuildId;
use sqlx::PgPool;

mod structs;
use structs::*;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;


#[derive(Clone)]
pub struct Data(pub Arc<DataInner>);

impl std::ops::Deref for Data {
    type Target = DataInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct DataInner {
    pub database: PgPool,
    pub guild_co: DashMap<GuildId, GuildConfig>,
    pub time_started: std::time::Instant,
}

impl Data {
    pub async fn get_guild(&self, id: GuildId) -> GuildConfig {
        if let Some(cached_conf) = self.guild_co.get(&id) {
            cached_conf.clone()
        } else {
            self._get_guild_postgres(id).await
        }
    }

    pub async fn save_guild(&self, id: GuildId, config: GuildConfig) {
        self.guild_co.insert(id, config.clone());

        if config != GuildConfig::default() {
            self._insert_guild(id, config).await
        }
    }

    pub fn drop_guild_caches(&self, id: GuildId) {
        self.guild_co.remove(&id);
    }

    pub async fn _delete_guild(&self, id: GuildId) {
        self.drop_guild_caches(id);
        self._delete_guild_postgres(id).await;
    }

    async fn _delete_guild_postgres(&self, id: GuildId) {
        let _ = sqlx::query!("DELETE FROM guilds WHERE guild_id = $1", i64::from(id))
            .execute(&self.database)
            .await;
    }

    async fn _insert_guild(&self, id: GuildId, config: GuildConfig) {
        let _ = sqlx::query!(
            "INSERT INTO guilds (guild_id, prefix) VALUES ($1, $2) ON CONFLICT (guild_id) DO UPDATE SET prefix = $2",
            i64::from(id),
            config.prefix
        )
    .execute(&self.database)
        .await;
    }

    async fn _get_guild_postgres(&self, id: GuildId) -> GuildConfig {
        let result = sqlx::query!("SELECT * FROM guilds WHERE guild_id = $1", i64::from(id))
            .fetch_one(&self.database)
            .await;

        match result {
            Ok(record) => {
                // set mod config
                GuildConfig::new(record.prefix)
            }
            Err(error) => match error {
                sqlx::Error::RowNotFound => {
                    // create new, cache.
                    GuildConfig::default()
                }

                _ => GuildConfig::default(),
            },
        }
    }
}
