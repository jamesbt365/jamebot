use rand::{seq::SliceRandom, thread_rng};
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use crate::util::parse_id;

const HUGS: &[&str] = &[
    "https://cdn.discordapp.com/attachments/706370092283461695/1048732201040224336/6c4d1a89eb9bcf38bcba1ae379275384.jpg",
    "https://cdn.discordapp.com/attachments/706370092283461695/1048732201275113513/00fc0198b5669801a4549fb37d95ca5b.jpg",
    "https://cdn.discordapp.com/attachments/706370092283461695/1048732201702924388/0ce07377d31ab045a0dc35810832e296.jpg",
    "https://cdn.discordapp.com/attachments/706370092283461695/1048732201975562250/01fbbe021f6de127947b0b3d8cef0f28.jpg",
    "https://cdn.discordapp.com/attachments/706370092283461695/1048732202327875724/1ca7b7244fc002d31a5c0b3a80a5e6e3.jpg",
    "https://cdn.discordapp.com/attachments/706370092283461695/1048732202600513646/9ed8e672cdc7c9b87dba7af1ec965a0a.jpg",
    "https://cdn.discordapp.com/attachments/706370092283461695/1048732202868932678/31d3117eaefdd4bada76573cd1860baf.jpg",
    "https://cdn.discordapp.com/attachments/872953485296009316/1048735119835402302/1578763433581.png",
    "https://cdn.discordapp.com/attachments/872953485296009316/1048735120179343381/1630541183104.jpg",
    "https://cdn.discordapp.com/attachments/872953485296009316/1048735120468738098/1637626165611.jpg",
    "https://cdn.discordapp.com/attachments/872953485296009316/1048735120716206162/1638486503625.png",
    "https://cdn.discordapp.com/attachments/872953485296009316/1048735121047564328/1642977586623.jpg",
    "https://cdn.discordapp.com/attachments/872953485296009316/1048735121286627358/1643838342089.png",
    "https://cdn.discordapp.com/attachments/872953485296009316/1048735121584427080/1643620971342.jpg",
];

#[command]
async fn hug(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    
    let target_id = match args.single::<String>().ok().and_then(parse_id) {
        Some(id) => UserId(id),
        None => {
            msg.channel_id.say(&ctx, "You can't hug nobody.").await?;

            return Ok(());
        }
    };
    
    let target_user = match target_id.to_user(&ctx).await {
        Ok(u) => u,
        Err(_) => {
            msg.channel_id.say(&ctx, "Failed to fetch user, are you selecting a valid user?").await?;

            return Ok(());
        }
    };

    let hug = {
        let mut rng = thread_rng();
        format!("{}", HUGS.choose(&mut rng).unwrap())
    };

    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.title(format!(
                    "{} hugged {}!",
                    msg.author.name,
                    target_user.name
                )); 
                e.image(&hug);
                e.colour(0x2F3136);

                e.author(|a| {
                    a.name(&target_user.name);
                    a.icon_url(target_user.face());

                    a
                });
                e
            })
        })
        .await?;
    Ok(())
}
