use crate::{Context, Error};
use poise::serenity_prelude as serenity;
use rand::{prelude::Distribution, thread_rng};

/// Roll some dice
#[poise::command(
    prefix_command,
    slash_command,
    category = "Utility",
    user_cooldown = "5"
)]
pub async fn roll(
    ctx: Context<'_>,
    #[description = "Die size"] die_size: Option<u64>,
    #[description = "Die count"] die_count: Option<u64>,
) -> Result<(), Error> {
    let die_size = die_size.unwrap_or(6);
    let die_count = die_count.unwrap_or(1);
    // possible to add delimiter?

    let roll: u64 = {
        let mut rng = thread_rng();
        let uniform = rand::distributions::Uniform::new_inclusive(1, die_size);
        (0..die_count).map(|_| uniform.sample(&mut rng)).sum()
    };

    ctx.send(
        poise::CreateReply::default().embed(serenity::CreateEmbed::default().description(format!(
            "Rolling {die_size}d{die_count} for **{roll}**! :game_die:"
        ))),
    )
    .await?;

    Ok(())
}
