use rand::{prelude::Distribution, thread_rng};
use crate::{Context, Error};

/// Roll some dice
#[poise::command(prefix_command, slash_command, category = "Utility", user_cooldown="5")]
pub async fn roll(
    ctx: Context<'_>,
    #[description = "Die size"] die_size: Option<u32>,
    #[description = "Die count"] die_count: Option<u16>,
) -> Result<(), Error> {
    let die_size = die_size.unwrap_or(6);
    let die_count = die_count.unwrap_or(1);
    // possible to add delimiter?

    let roll: u64 = {
        let mut rng = thread_rng();
        let uniform = rand::distributions::Uniform::new_inclusive(1, die_size);
        (0..die_count)
            .map(|_| uniform.sample(&mut rng) as u64)
            .sum()
    };

    ctx.send(|e| {
        e.embed(|e|
            e.description(format!(
                "Rolling {}d{} for **{}**! :game_die:",
                die_size, die_count, roll
            ))
        )
    })
    .await?;

    Ok(())
}
