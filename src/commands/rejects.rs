use std::process::Stdio;

use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

use execute::Execute;

#[command]
#[owners_only]
async fn test(ctx: &Context, msg: &Message) -> CommandResult {

    let mut command = execute::command!("free -h");
    command.stdout(Stdio::piped());
    let output = command.execute_output().unwrap();
    msg.channel_id.say(&ctx.http, String::from_utf8(output.stdout).unwrap()).await?;
    Ok(())
}