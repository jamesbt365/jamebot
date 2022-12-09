
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::Context,
};

use reqwest::Client as ReqwestClient;
use serde::Deserialize;

#[derive(Deserialize)]
struct Respon {
    list: Vec<Definition>,
}

#[derive(Deserialize)]
struct Definition {
    definition: String,
    example: String,
    word: String,
    thumbs_up: u32,
    thumbs_down: u32,
    permalink: String,
}


#[command]
#[bucket(fun)]
#[description("Get the definition of a word or phrase.")]
async fn urban(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if args.rest().is_empty() {
        msg.channel_id.say(ctx, "Please provide a word or phase to lookup.").await?;
        return Ok(());
    }

    let term = args.rest();

    let reqwest = ReqwestClient::new();
    let response = reqwest.get("https://api.urbandictionary.com/v0/define").query(&[("term", term)]).send().await?.json::<Respon>().await?;
    if response.list.is_empty() {
        msg.channel_id.say(ctx, format!("No definitions found for `{term}`. Try a different word.")).await?;
        return Ok(());
    }

        let choice = &response.list[0];
        if let Err(why) = msg
            .channel_id
            .send_message(ctx, |m| {
                m.embed(|e| {
                    e.title(&choice.word);
                    e.url(&choice.permalink);
                    e.description(format!(
                        "**Definition:**\n{}\n\n **Example:**\n{}\n\n",
                        &choice.definition, &choice.example
                    ));
                    e.field("👍", choice.thumbs_up, true);
                    e.field("👎", choice.thumbs_down, true);
                    e
                });
                m
            })
            .await
        {
            if "Embed too large." == why.to_string() {
                msg.channel_id.say(ctx, &choice.permalink).await?;
            } else {
                msg.channel_id.say(ctx, why).await?;
                return Ok(());
            }
        };

    Ok(())
}
