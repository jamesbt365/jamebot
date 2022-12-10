use serenity::framework::standard::macros::group;

mod hug;
mod urban;
mod search;

use self::{hug::*, urban::*, search::*};
#[group]
#[commands(hug, urban, ddg)]
#[only_in("guild")]
pub struct Fun;
