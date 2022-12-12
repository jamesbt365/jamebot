use serenity::framework::standard::macros::command;
use serenity::framework::standard::{CommandResult, Args};
use serenity::model::prelude::*;
use serenity::prelude::Context;

use reqwest::Url;
#[command]
#[min_args(1)]
#[aliases(duckduckgo, search)]
#[description("use https://lmddgtfy.net/ to search something up.")]
async fn ddg(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let url = Url::parse_with_params("https://lmddgtfy.net/", &[("q", args.message())])?;
    msg.channel_id.say(ctx, url).await?;

    Ok(())
}
