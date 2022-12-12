use serenity::framework::standard::macros::group;

mod urban;
mod search;

use self::{urban::*, search::*};
#[group]
#[commands(urban, ddg)]
pub struct Utility;
