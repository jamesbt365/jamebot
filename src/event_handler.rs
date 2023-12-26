use crate::{Data, Error};
use poise::serenity_prelude as serenity;

pub async fn event_handler(
    event: serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        // Eventually just do this all in GuildCreate and check on message, its generally the easiest method.
        serenity::FullEvent::CacheReady { ctx: _, guilds } => {
            for guild in guilds {
                data.get_guild(guild).await;
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
