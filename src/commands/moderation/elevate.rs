
use crate::{Context, Error};
use poise::serenity_prelude::{self as serenity, Member, Role};

/// Elevate permissions temporarily
#[poise::command(slash_command, guild_only, category = "Elevation", required_bot_permissions = "MANAGE_ROLES")]
pub async fn elevate(
    ctx: Context<'_>,
    #[description = "The user to elevate."] mut member: Member,
    #[description = "The role to elevate to."] role: Role,
    #[description = "The time to elevate for."] duration: u16,
) -> Result<(), Error> {
    if elevate_checker(ctx, member, role, duration).await.is_ok() {
        // do elevation and confirm msg.
    };


    Ok(())
}
// can we error to a function to do a nice oneliner with everything kept in one place?
/// Elevation eligibility checker.
async fn elevate_checker(ctx: Context<'_>, member: Member, role: Role, duration: u16) -> Result<(), ElevateCheckError> {
    let data = ctx.data();
    let config = ctx.data().get_guild(ctx.guild_id().unwrap()).await;

    if let Some(moderation) = config.modules.mod_config {
        if let Some(elevate) = moderation.elevation_settings {
            if !elevate.enabled {
                ctx.send(poise::CreateReply::new().embed(serenity::CreateEmbed::new().title("Error executing command").description("Elevation is not enabled in the moderation configuration!"))).await?;
                return Err(ElevateCheckError::ElevationDisabled)
            }

            // if no max time is set, set to the default limit.
            if elevate.max_time.unwrap_or(3600) < duration {
                ctx.send(poise::CreateReply::new().embed(serenity::CreateEmbed::new().title("Error executing command").description("above duration limit."))).await?;
                return Err(ElevateCheckError::DurationTooLarge)
            }

            // check if user is already elevated (against multi), set amount if allowed, grab info?.

            // check if executing user is elevated. (this is a no no)


        }
    } else {
        ctx.send(poise::CreateReply::new().embed(serenity::CreateEmbed::new().title("Error executing command").description("Moderation is not enabled in the guild configuration!"))).await?;
        return Err(ElevateCheckError::NoModConfig)
    }


    Ok(())
}

#[derive(Debug)]
enum ElevateCheckError {
    SerenityError(String),
    NoModConfig,
    ElevationDisabled,
    DurationTooLarge,
    UserAlreadyElevated,
    ExecuterAlreadyElevated,
}

impl From<serenity::Error> for ElevateCheckError {
    fn from(error: serenity::Error) -> Self {
        ElevateCheckError::SerenityError(error.to_string())
    }
}

