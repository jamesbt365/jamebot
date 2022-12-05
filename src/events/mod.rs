
use serenity::model::prelude::Message;
use serenity::prelude::Context;

pub mod emotes;

pub async fn message(ctx: &Context, msg: &Message) {
  emotes::message(&ctx, &msg).await;
}
