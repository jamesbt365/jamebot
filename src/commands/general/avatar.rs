use crate::{Context, Error};
use poise::serenity_prelude as serenity;

/// Display a members avatar
#[poise::command(prefix_command, slash_command, category = "General")]
pub async fn avatar(
    ctx: Context<'_>,
    #[description = "The user's avatar to display"] user: serenity::User,
) -> Result<(), Error> {

    ctx.send(|e| {
        e.embed(|e|
            e.title(format!("{}{}'s avatar:", user.name, user.discriminator))
            .image(user.face())
        )
    })
    .await?;

    Ok(())
}