use std::time::Duration;

use crate::{Context, Error};
use poise::serenity_prelude as serenity;

/// Manage the Server Configuration!
#[poise::command(
    prefix_command,
    slash_command,
    category = "Configuration",
    guild_cooldown = "5",
    guild_only
)]
pub async fn settings(ctx: Context<'_>) -> Result<(), Error> {
    let guild_config = ctx.data().get_guild(ctx.guild_id().unwrap()).await;


    let prefix_str = if let Some(prefix) = guild_config.prefix {
        format!("`{}`", prefix)
    } else {
        "None".to_string()
    };

    let embed = serenity::CreateEmbed::default()
        .title("Guild Settings")
        .field("Prefix", prefix_str, true);
    ctx.send(poise::CreateReply::default().embed(embed)).await?;

    Ok(())
}

// This command will eventually be configurable to who accesses it.
/// Manage the server configuration!
#[poise::command(
    rename = "modify-guild-settings",
    prefix_command,
    slash_command,
    category = "Configuration",
    guild_cooldown = "5",
    guild_only
)]
pub async fn change_settings(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let guild_config = ctx.data().get_guild(guild_id).await;

    let prefix_str = if let Some(prefix) = guild_config.clone().prefix {
        format!("`{}`", prefix)
    } else {
        "None".to_string()
    };

    let embed = serenity::CreateEmbed::default()
        .title("Guild Settings")
        .field("Prefix", prefix_str, true);

    let ctx_id = ctx.id();
    let prefix_id = format!("{}modal", ctx.id());
    let thing = serenity::CreateActionRow::Buttons(vec![
        serenity::CreateButton::new(&prefix_id).label("change prefix")
    ]);

    let builder = poise::CreateReply::default()
        .embed(embed.clone())
        .components(vec![thing]);
    let msg = ctx.send(builder.clone()).await?;

    while let Some(press) = serenity::ComponentInteractionCollector::new(ctx)
        .filter(move |press| press.data.custom_id.starts_with(&ctx_id.to_string()))
        .timeout(Duration::from_secs(180))
        .await
    {
        if press.data.custom_id == prefix_id {
            let data = poise::execute_modal_on_component_interaction::<NewPrefix>(
                ctx,
                std::sync::Arc::new(press.clone()),
                None,
                None,
            )
            .await?;
            if let Some(data) = data {
                let prefix_str = format!("`{}`", data.input.clone());
                // should validate it.

                let new_conf = guild_config.clone().prefix(Some(data.input));
                ctx.data().save_guild(guild_id, new_conf).await;

                let embed = serenity::CreateEmbed::default()
                    .title("Guild Settings")
                    .field("Prefix", prefix_str, true);
                msg.edit(ctx, poise::CreateReply::default().embed(embed))
                    .await?;
            }
        }
    }

    Ok(())
}

#[derive(Debug, poise::Modal)]
struct NewPrefix {
    input: String,
}
