use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Pong!").await?;
    Ok(())
}


#[command]
async fn source(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "<https://github.com/jamesbt365/jamebot>").await?;
    Ok(())
}