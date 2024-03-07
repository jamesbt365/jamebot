pub mod colour;
pub mod roll;

#[must_use]
pub fn commands() -> Vec<crate::Command> {
    {
        colour::commands()
            .into_iter()
            .chain(roll::commands())
            .collect()
    }
}
