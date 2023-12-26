
use crate::{Context, Error};
use poise::serenity_prelude::{Member, Role};

/// Elevate permissions temporarily
#[poise::command(slash_command, category = "Elevation")]
pub async fn elevate(
    ctx: Context<'_>,
    #[description = "The user to elevate."] mut member: Member,
    #[description = "The role to elevate to."] role: Role,
    #[description = "The time to elevate for."] duration: String,
) -> Result<(), Error> {
    let _ = member.add_role(ctx, role.id).await;
    let _ = duration;
    Ok(())
}
