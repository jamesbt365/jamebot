use crate::{Context, Error};
use poise::serenity_prelude as serenity;

/// Displays the user's info
#[poise::command(prefix_command, slash_command, aliases("uinfo"), category = "General")]
pub async fn userinfo(
    ctx: Context<'_>,
    #[description = "User"] user: Option<serenity::User>,
) -> Result<(), Error> {

    let user = user.as_ref().unwrap_or_else(|| ctx.author());
    let guild = ctx.partial_guild().await.unwrap();
    let member = guild.member(ctx, user.id).await.unwrap();

    let nick = user
    .nick_in(&serenity::CacheHttp::http(&ctx), ctx.guild().unwrap().id)
    .await
    .unwrap_or_else(|| user.name.to_string());

    let roles = member.roles(ctx).unwrap_or_default().into_iter();
    let mut user_roles = roles
        .map(|r| format!("<@&{}>", *r.id.as_u64()))
        .collect::<Vec<_>>()
        .join(" ");

    if user_roles.is_empty() {
        user_roles = "@everyone".to_string();
    }

    let registered = format!("<t:{}:R>", user.created_at().timestamp());
    let joined_server = format!("<t:{}:R>", member.joined_at.unwrap().timestamp());


    ctx.send(|e| {
        e.embed(|e|
            // TODO: add online status
            e.title(format!("{}#{} (ID: {})", user.name, user.discriminator, user.id))
            .thumbnail(user.face())
            .field("Nickname:", nick, false)
            .field("Registered:", registered, true)
            .field("Joined:", joined_server, true)
            .field("Roles:", user_roles, false)
        )
        // TODO: add user accent colour.
    })
    .await?;

    Ok(())
}
