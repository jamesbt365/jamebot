use serenity::model::prelude::Message;
use serenity::prelude::Context;
use serenity::model::prelude::{EmojiIdentifier, EmojiId};

pub async fn message(ctx: &Context, msg: &Message) {
    // there is a better way of doing this expect it is midnight and I don't want to figure out how.
  if msg.content.to_lowercase().contains("lolicamera")
  {
      if let Err(why) = msg.react(&ctx.http, EmojiIdentifier { animated: false, id: EmojiId(916511786159718460), name: "lolicamera".to_string() }).await {
println!("Error sending message: {:?}", why);
      }
  };

  if msg.content.to_lowercase().contains("lencamera",)
  {
      if let Err(why) = msg.react(&ctx.http, EmojiIdentifier { animated: false, id: EmojiId(999659904811933727), name: "lencamera".to_string() }).await {
println!("Error sending message: {:?}", why);
      }
  };

  if msg.content.to_lowercase().contains("morbius",)
  {
      if let Err(why) = msg.react(&ctx.http, EmojiIdentifier { animated: false, id: EmojiId(1028320732649893888), name: "morbius".to_string() }).await {
println!("Error sending message: {:?}", why);
      }
  };

  if msg.content.to_lowercase().contains("fag",)
  {
      if let Err(why) = msg.react(&ctx.http, EmojiIdentifier { animated: false, id: EmojiId(1028326126008930374), name: "fag".to_string() }).await {
println!("Error sending message: {:?}", why);
      }
  };
// shit code complete !!
}
