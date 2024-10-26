use crate::{Context, Error};
use poise::serenity_prelude::{self as serenity, Permissions, Role};

fn bool_converter(b: bool) -> String {
    if b {
        "Yes".to_string()
    } else {
        "No".to_string()
    }
}

/// Check the info about a specific role!
#[poise::command(
    aliases("role_info", "role-info"),
    slash_command,
    prefix_command,
    guild_only,
    category = "Utility",
    track_edits,
    install_context = "Guild|User",
    interaction_context = "Guild|BotDm|PrivateChannel",
    user_cooldown = "5"
)]
pub async fn roleinfo(
    ctx: Context<'_>,
    #[description = "Role"] role: Role,
    #[description = "Show key permissions or all?"]
    #[rename = "permissions"]
    show_all_permissions: Option<bool>,
) -> Result<(), Error> {
    let role_id = role.id.get().to_string();
    let colour = format!("#{}", role.colour.hex());
    let mention = format!("`<@&{role_id}>`");
    let hoisted = bool_converter(role.hoist());
    let mentionable = bool_converter(role.mentionable());

    let managed = bool_converter(role.managed());
    let role_name = role.name;

    let permissions = role.permissions;
    let permissions_title = if let Some(true) = show_all_permissions {
        "Permissions".to_string()
    } else {
        "Key Permissions".to_string()
    };

    let key_permissions = [
        Permissions::ADMINISTRATOR,
        Permissions::MANAGE_GUILD,
        Permissions::KICK_MEMBERS,
        Permissions::BAN_MEMBERS,
        Permissions::MANAGE_CHANNELS,
        Permissions::MANAGE_GUILD,
        Permissions::VIEW_AUDIT_LOG,
        Permissions::PRIORITY_SPEAKER,
        Permissions::SEND_TTS_MESSAGES,
        Permissions::MANAGE_MESSAGES,
        Permissions::MENTION_EVERYONE,
        Permissions::VIEW_GUILD_INSIGHTS,
        Permissions::MUTE_MEMBERS,
        Permissions::DEAFEN_MEMBERS,
        Permissions::MOVE_MEMBERS,
        Permissions::MANAGE_ROLES,
        Permissions::MANAGE_WEBHOOKS,
        Permissions::MANAGE_GUILD_EXPRESSIONS,
        Permissions::MANAGE_THREADS,
        Permissions::CREATE_PRIVATE_THREADS,
        Permissions::MODERATE_MEMBERS, // Timeout
        Permissions::VIEW_CREATOR_MONETIZATION_ANALYTICS,
    ];

    // I can 100% reduce allocations if I try hard enough.
    let formatted_permissions: Vec<String> = permissions
        .iter()
        .filter_map(|permission| {
            if show_all_permissions.unwrap_or(false) || key_permissions.contains(&permission) {
                Some(if key_permissions.contains(&permission) {
                    // Highlight key permissions
                    format!("**{permission}**")
                } else {
                    format!("{permission}")
                })
            } else {
                None
            }
        })
        .collect();

    let permissions_list = formatted_permissions.join(", ");

    let timestamp = role.id.created_at().format("%Y/%m/%d %H:%M");
    println!("{:?}", role.unicode_emoji);

    let embed = serenity::CreateEmbed::default()
        .field("ID", role_id, true)
        .field("Name", role_name, true)
        .field("Colour", colour, true)
        .field("Mention", mention, true)
        .field("Hoisted", hoisted, true)
        .field("Mentionable", mentionable, true)
        .field("Managed", managed, true)
        .field(permissions_title, permissions_list, false)
        .footer(serenity::CreateEmbedFooter::new(format!(
            "Role Created • {timestamp}"
        )));

    let message = poise::CreateReply::default().embed(embed);
    ctx.send(message).await?;
    Ok(())
}

#[must_use]
pub fn commands() -> [crate::Command; 1] {
    [roleinfo()]
}
