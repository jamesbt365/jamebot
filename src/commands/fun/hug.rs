use crate::{Context, Error};
use poise::serenity_prelude as serenity;

use rand::{seq::SliceRandom, thread_rng};

const HUGS: &[&str] = &[
    // Link's additions
    "https://cdn.discordapp.com/attachments/1050568393251623023/1050569361829666856/6c4d1a89eb9bcf38bcba1ae379275384.jpg",
    "https://cdn.discordapp.com/attachments/1050568393251623023/1050569361456377856/00fc0198b5669801a4549fb37d95ca5b.jpg",
    "https://cdn.discordapp.com/attachments/1050568393251623023/1050569361066315867/0ce07377d31ab045a0dc35810832e296.jpg",
    "https://cdn.discordapp.com/attachments/1050568393251623023/1050569360730759259/01fbbe021f6de127947b0b3d8cef0f28.jpg",
    "https://cdn.discordapp.com/attachments/1050568393251623023/1050569360730759259/01fbbe021f6de127947b0b3d8cef0f28.jpg",
    "https://cdn.discordapp.com/attachments/1050568393251623023/1050569348181397604/9ed8e672cdc7c9b87dba7af1ec965a0a.jpg",
    "https://cdn.discordapp.com/attachments/1050568393251623023/1050569347883597934/31d3117eaefdd4bada76573cd1860baf.jpg",
    "https://cdn.discordapp.com/attachments/1050568393251623023/1050569347581616219/1578763433581.png",
    "https://cdn.discordapp.com/attachments/1050568393251623023/1050569347258650674/1630541183104.jpg",
    "https://cdn.discordapp.com/attachments/1050568393251623023/1050569346956656820/1637626165611.jpg",
    "https://cdn.discordapp.com/attachments/1050568393251623023/1050569346595954698/1638486503625.png",
    "https://cdn.discordapp.com/attachments/1050568393251623023/1050569346197491782/1642977586623.jpg",
    "https://cdn.discordapp.com/attachments/1050568393251623023/1050569345853554768/1643838342089.png",
    "https://cdn.discordapp.com/attachments/1050568393251623023/1050569345568350218/1643620971342.jpg",
    // Trash's additions
    "https://cdn.discordapp.com/attachments/1050568393251623023/1050571812355973171/Hugging_002.jpg",
    "https://cdn.discordapp.com/attachments/1050568393251623023/1050571812016246896/Hugging_003.jpg",
    "https://cdn.discordapp.com/attachments/1050568393251623023/1050571811659726910/Hugging_004.jpg",
    "https://cdn.discordapp.com/attachments/1050568393251623023/1050571811210928178/Hugging_005.png",
    "https://cdn.discordapp.com/attachments/1050568393251623023/1050571810862813234/Hugging_006.jpg",
    "https://cdn.discordapp.com/attachments/1050568393251623023/1050571810535653436/Hugging_007.jpg",
    "https://cdn.discordapp.com/attachments/1050568393251623023/1050571810212683786/Hugging_008.jpg",
    "https://cdn.discordapp.com/attachments/1050568393251623023/1050572272085254195/Hugging_010.jpg",
    "https://cdn.discordapp.com/attachments/1050568393251623023/1050572318516183091/Hugging_012.jpg",
    "https://cdn.discordapp.com/attachments/1050568393251623023/1050572442910871582/Hugging_014.jpg",
    "https://cdn.discordapp.com/attachments/1050568393251623023/1050572442369794138/Hugging_015.jpg",
    "https://cdn.discordapp.com/attachments/1050568393251623023/1050572441891635210/Hugging_016.jpg",
    "https://cdn.discordapp.com/attachments/1050568393251623023/1050572441547722833/Hugging_017.jpg",
    "https://cdn.discordapp.com/attachments/1050568393251623023/1050572441149251635/Hugging_018.jpg",
    "https://cdn.discordapp.com/attachments/1050568393251623023/1050572440197144617/Hugging_021.jpg",
    "https://cdn.discordapp.com/attachments/1050568393251623023/1050572439643488286/Hugging_022.jpg",
    "https://cdn.discordapp.com/attachments/1050568393251623023/1050572438792065105/Hugging_024.jpg",
    "https://cdn.discordapp.com/attachments/1050568393251623023/1050572881597304832/Hugging_026.jpg",
    "https://cdn.discordapp.com/attachments/1050568393251623023/1050572881186279545/Hugging_027.jpg",
    "https://cdn.discordapp.com/attachments/1050568393251623023/1050572880787804170/Hugging_028.jpg",
    // Phoenix's additions
    "https://media.discordapp.net/attachments/829787267992256562/1050960293745922150/image0.jpg",
    "https://media.discordapp.net/attachments/829787267992256562/1050960752908972132/image0.jpg",
    "https://media.discordapp.net/attachments/829787267992256562/1050960971029545021/image0.jpg",
    "https://media.discordapp.net/attachments/829787267992256562/1050961267050942584/image0.jpg",
    "https://media.discordapp.net/attachments/829787267992256562/1050961805398245437/image0.jpg",
];
/// Hug your friends :3
#[poise::command(prefix_command, slash_command, category = "Fun", user_cooldown = "4")]
pub async fn hug(
    ctx: Context<'_>,
    #[description = "The user to hug!! :3"] user: serenity::User,
) -> Result<(), Error> {

    let hug = {
        let mut rng = thread_rng();
        format!("{}", HUGS.choose(&mut rng).unwrap())
    };


    ctx.send(|e| {
        e.embed(|e|
            e.title(format!("{} hugged {}!", ctx.author().name, user.name))
            .image(&hug)
            // maybe add the author thing later
        )
    })
    .await?;

    Ok(())
}
