use crate::{Data, Error};
use poise::serenity_prelude::{self as serenity, FullEvent};

pub async fn event_handler(
    framework: poise::FrameworkContext<'_, Data, Error>,
    event: &serenity::FullEvent,
) -> Result<(), Error> {
    let data = framework.user_data();
    match event {
        // Eventually just do this all in GuildCreate and check on message, its generally the easiest method.
        FullEvent::CacheReady { guilds } => {
            for guild in guilds {
                data.get_guild(*guild).await;
            }
        }
        FullEvent::GuildDelete {
            incomplete,
            full: _,
        } => {
            if !incomplete.unavailable {
                data.drop_guild_caches(incomplete.id);
            }
        }

        _ => (),
    }

    Ok(())
}
