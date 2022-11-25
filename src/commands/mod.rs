use serenity::framework::standard::macros::group;

pub mod meta;

use self::meta::*;
#[group]
#[commands(ping, source, shutdown, test)]
pub struct Meta;