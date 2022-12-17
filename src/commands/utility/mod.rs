use serenity::framework::standard::macros::group;

mod urban;
mod search;
mod roll;

use self::{urban::*, search::*, roll::*};
#[group]
#[commands(urban, ddg, roll)]
pub struct Utility;
