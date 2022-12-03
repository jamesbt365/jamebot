use serenity::framework::standard::macros::group;

mod hug;

use self::hug::*;

#[group]
#[commands(hug)]
#[only_in("guild")]
pub struct Fun;