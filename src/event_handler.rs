use crate::{config, Data, Error, GuildConfig};
use poise::serenity_prelude as serenity;

pub async fn event_handler(
    event: serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        // Eventually just do this all in GuildCreate and check on message, its generally the easiest method.
        serenity::FullEvent::CacheReady { ctx: _, guilds } => {
            let guild_cache = &data.guild_cache;
            let database = &data.database;

            for guild in guilds {
                let existing_guild = sqlx::query!(
                    "SELECT * FROM guilds WHERE guild_id = $1",
                    guild.get() as i64
                )
                .fetch_optional(database)
                .await?;

                if let Some(row) = existing_guild {
                    // gonna leave the init code here.
                    let guild_config = GuildConfig::new(row.prefix);
                    guild_cache.insert(guild, guild_config);
                } else {
                    // Guild doesn't exist in the database (guild joined while offline.)
                    config::add_guild_config_def(data, guild).await;
                }
            }
        }
        serenity::FullEvent::GuildDelete { ctx: _, incomplete, full: _ } => {
            if !incomplete.unavailable {
                data.drop_guild_cache(incomplete.id);
            }
        }

        _ => (),
    }

    Ok(())
}
