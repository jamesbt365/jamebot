use crate::{config, Data, Error, GuildConfig};
use poise::serenity_prelude::{self as serenity, EmojiId, ReactionType};
use std::collections::HashMap;

pub async fn event_handler(
    event: serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    let keyword_reactions: HashMap<&str, ReactionType> = {
        let mut map = HashMap::new();
        map.insert(
            "lolicamera",
            ReactionType::Custom {
                animated: false,
                id: EmojiId::new(916511786159718460),
                name: Some("lolicamera".to_string()),
            },
        );
        map.insert(
            "fag",
            ReactionType::Custom {
                animated: false,
                id: EmojiId::new(1028326126008930374),
                name: Some("fag".to_string()),
            },
        );
        map.insert(
            "morbius",
            ReactionType::Custom {
                animated: false,
                id: EmojiId::new(1028320732649893888),
                name: Some("morbius".to_string()),
            },
        );
        let bitlink = ReactionType::Custom {
            animated: false,
            id: EmojiId::new(901179398093422643),
            name: Some("32bitlink".to_string()),
        };
        map.insert("32bitlink", bitlink.clone());
        map.insert("32 bit link", bitlink.clone());

        let rimokon = ReactionType::Custom {
            animated: false,
            id: EmojiId::new(1069331591953911848),
            name: Some("rimokon".to_string()),
        };
        map.insert("rimokon", rimokon.clone());
        map.insert("remote", rimokon.clone());
        map.insert("control", rimokon.clone());
        map.insert("sit down please yeah", rimokon.clone());
        map
    };
    match event {
        serenity::FullEvent::Message { ctx, new_message } => {
            let content_lowercase = new_message.content.to_lowercase();
            for (keyword, reaction) in keyword_reactions.iter() {
                if content_lowercase.contains(keyword) {
                    new_message.react(&ctx, reaction.clone()).await?;
                }
            }
        }
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
        serenity::FullEvent::GuildCreate {
            ctx,
            guild,
            is_new: Some(new),
        } => {
            if new {
                config::cache_guild_config(data, guild.id).await;
            }
        }
        serenity::FullEvent::GuildDelete { ctx: _, incomplete, full: _ } => {
            if !incomplete.unavailable {
                config::drop_guild_cache(data, incomplete.id).await;
            }
        }

        _ => (),
    }

    Ok(())
}
