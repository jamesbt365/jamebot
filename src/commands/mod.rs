pub mod fun;
pub mod general;
pub mod guild_config;
pub mod meta;
pub mod moderation;
pub mod utility;

#[must_use]
pub fn commands() -> Vec<crate::Command> {
    meta::commands()
        .into_iter()
        .chain(utility::commands())
        .chain(fun::commands())
        .collect()
}
