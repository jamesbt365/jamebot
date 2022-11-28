use serenity::framework::standard::macros::group;

pub mod meta;
pub mod rejects;
use self::{meta::*, rejects::*};
#[group]
#[commands(ping, source, shutdown, test)]
pub struct Meta;