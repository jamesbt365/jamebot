use std::num::NonZeroU16;

use crate::{Context, Error};
use poise::serenity_prelude as serenity;

/// Display a members avatar
#[poise::command(prefix_command, slash_command, category = "General")]
pub async fn avatar(
    ctx: Context<'_>,
    #[description = "The user's avatar to display"] user: serenity::User,
) -> Result<(), Error> {
    // improve later.
    let title = format!(
        "{}#{}'s avatar:",
        user.name,
        user.discriminator.unwrap_or(NonZeroU16::new(1).unwrap())
    );
    let embed = serenity::CreateEmbed::default()
        .title(title)
        .image(user.face());
    ctx.send(poise::CreateReply::default().embed(embed)).await?;

    Ok(())
}
