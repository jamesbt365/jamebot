use serenity::framework::standard::macros::group;

pub mod meta;

use self::meta::*;
#[group]
#[commands(ping, source)]
pub struct Meta;