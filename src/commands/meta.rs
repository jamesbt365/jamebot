use std::collections::HashSet;

use serenity::framework::standard::macros::{help, command};
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::client::Context;
use serenity::framework::standard::{
    help_commands,
    Args,
    CommandGroup,
    HelpOptions,
};


use crate::ShardManagerContainer;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Pong!").await?;
    Ok(())
}


#[command]
#[description("Sends a link to my source code.")]
async fn source(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "<https://github.com/jamesbt365/jamebot>").await?;
    Ok(())
}

#[command]
#[owners_only]
async fn shutdown(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;

    if let Some(manager) = data.get::<ShardManagerContainer>() {
        msg.reply(ctx, "** Bailing out, you are on your own. Good luck.**").await?;
        manager.lock().await.shutdown_all().await;
    } else {
        msg.reply(ctx, "There was a problem getting the shard manager").await?;

        return Ok(());
    }

    Ok(())
}


#[help]
#[individual_command_tip = "Hello! If you would like more information on a command or group, pass them as an argument.\n"]
#[strikethrough_commands_tip_in_guild = "~~`Strikethrough commands`~~ are not available because the bot can't run them."]
#[strikethrough_commands_tip_in_dm = "~~`Strikethrough commands`~~ are not available because the bot can't run them."]
#[max_levenshtein_distance(3)]
#[lacking_permissions = "Hide"]
#[lacking_role = "Hide"]
#[wrong_channel = "Strike"]
async fn help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}
