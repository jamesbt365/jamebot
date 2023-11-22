use crate::{Context, Error};

// Post the link to the bots source code.
#[poise::command(slash_command, prefix_command, category = "Meta")]
pub async fn source(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("<https://github.com/jamesbt365/jamebot>").await?;
    Ok(())
}

/// Shutdown the bot gracefully.
#[poise::command(prefix_command, owners_only, hide_in_help)]
pub async fn shutdown(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("**Bailing out, you are on your own. Good luck.**")
        .await?;
    ctx.framework().shard_manager().shutdown_all().await;
    Ok(())
}

/// Displays JameBot's uptime
#[poise::command(slash_command, prefix_command, category = "Meta")]
pub async fn uptime(ctx: Context<'_>) -> Result<(), Error> {
    let uptime = std::time::Instant::now() - ctx.data().time_started;

    let calculation = |a, b| (a / b, a % b);

    let seconds = uptime.as_secs();
    let (minutes, seconds) = calculation(seconds, 60);
    let (hours, minutes) = calculation(minutes, 60);
    let (days, hours) = calculation(hours, 24);

    ctx.say(format!(
        "`Uptime: {}d {}h {}m {}s`",
        days, hours, minutes, seconds
    ))
    .await?;

    Ok(())
}

/// Show general help or help to a specific command
// implement a custom one later.
#[poise::command(prefix_command, track_edits, slash_command, category = "Miscellaneous")]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), Error> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            ephemeral: true,
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}
