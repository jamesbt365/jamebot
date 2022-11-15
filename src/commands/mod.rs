use serenity::framework::standard::macros::group;

pub mod ping;

use self::ping::*;
#[group]
#[commands(ping)]
pub struct Test;