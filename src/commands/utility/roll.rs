use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::Context,
};
use rand::Rng;
#[command]
#[min_args(1)]
#[max_args(1)]
async fn roll(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {

    let range = args.single::<u32>().unwrap();
    // spits out an error when the value isn't a u32 but i literally do not care rn
    let random_number = rand::thread_rng().gen_range(1..range+1);

    msg.channel_id.say(&ctx, random_number.to_string()).await?;

    Ok(())
}
