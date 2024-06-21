pub mod colour;
pub mod info;
pub mod roll;
pub mod urban;

#[must_use]
pub fn commands() -> Vec<crate::Command> {
    {
        colour::commands()
            .into_iter()
            .chain(roll::commands())
            .chain(info::commands())
            .chain(urban::commands())
            .collect()
    }
}
