use serenity::framework::standard::macros::group;

pub mod meta;
pub mod rejects;

use self::{meta::*, rejects::*};
#[group]
#[commands(ping, source)]
pub struct Meta;


#[group]
#[commands(shutdown)]
pub struct Owner;

#[group]
#[commands(test)]
pub struct Rejects;