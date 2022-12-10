use serenity::framework::standard::macros::group;

pub mod meta;
pub mod fun;
pub mod rejects;

pub use self::{meta::*, rejects::*};
#[group]
#[commands(ping, source)]
pub struct Meta;

#[group]
#[commands(shutdown)]
#[help_available(false)]
pub struct Owner;

#[group]
#[commands(test)]
#[help_available(false)]
pub struct Rejects;
