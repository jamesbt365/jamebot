use crate::{Context, Error};
use poise::serenity_prelude as serenity;

/// Display a members avatar
#[poise::command(prefix_command, slash_command, category = "General")]
pub async fn avatar(
    ctx: Context<'_>,
    #[description = "The user's avatar to display"] user: serenity::User,
) -> Result<(), Error> {
    let title = format!(
        "{}'s avatar:",
        user.tag()
    );
    let embed = serenity::CreateEmbed::default()
        .title(title)
        .image(user.face());
    ctx.send(poise::CreateReply::default().embed(embed)).await?;

    Ok(())
}
