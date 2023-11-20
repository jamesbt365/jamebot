use crate::{Context, Error, Data};
use poise::serenity_prelude::{self as serenity, ReactionType, EmojiId};
use std::collections::HashMap;

pub async fn event_handler(
    ctx: &serenity::Context,
    event: &poise::Event<'_>,
    _ctx_poise: poise::FrameworkContext<'_, Data, Error>,
    _data: &Data,
) -> Result<(), Error> {
    let keyword_reactions: HashMap<&str, ReactionType> = {
        let mut map = HashMap::new();
        map.insert("lolicamera", ReactionType::Custom {
            animated: false,
            id: EmojiId(916511786159718460),
            name: Some("lolicamera".to_string()),
        });
        map.insert("fag", ReactionType::Custom {
            animated: false,
            id: EmojiId(1028326126008930374),
            name: Some("fag".to_string()),
        });
        map.insert("morbius", ReactionType::Custom {
            animated: false,
            id: EmojiId(1028320732649893888),
            name: Some("morbius".to_string()),
        });
        let bitlink = ReactionType::Custom {
            animated: false,
            id: EmojiId(901179398093422643),
            name: Some("32bitlink".to_string()),
        };
        map.insert("32bitlink", bitlink.clone());
        map.insert("32 bit link", bitlink.clone());

        let rimokon = ReactionType::Custom {
            animated: false,
            id: EmojiId(1069331591953911848),
            name: Some("rimokon".to_string()),
        };
        map.insert("rimokon", rimokon.clone());
        map.insert("remote", rimokon.clone());
        map.insert("control", rimokon.clone());
        map.insert("sit down please yeah", rimokon.clone());
        map
    };
    #[allow(clippy::single_match)] // yeah
    match event {
        poise::Event::Message { new_message } => {
            let content_lowercase = new_message.content.to_lowercase();
            for (keyword, reaction) in keyword_reactions.iter() {
                if content_lowercase.contains(keyword) {
                    new_message.react(ctx, reaction.clone()).await?;
                }
            }
        }
        _ => (),
    }

    Ok(())
}
