use crate::{Context, Error};
use poise::send_reply;

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

// TODO: Add option to switch to a different definition

/// Get the definition of a term on Urban Dictionary
#[poise::command(prefix_command, slash_command, category = "Utility")]
pub async fn urban(
    ctx: Context<'_>,
    #[description = "Query to search the definition of"] term: String,
) -> Result<(), Error> {

    let reqwest = ReqwestClient::new();
    let response = reqwest.get("https://api.urbandictionary.com/v0/define").query(&[("term", term.clone())]).send().await?.json::<Respon>().await?;
    if response.list.is_empty() {
        ctx.say(format!("No definitions found for `{term}`. Try a different word.")).await?;
        return Ok(());

    }

    let choice = &response.list[0];
    if let Err(why) = send_reply(ctx, |m| {
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
            // TODO: make this better.
        });
        m
    })
    .await
    {
        if "Embed too large." == why.to_string() {
            ctx.say(&choice.permalink).await?;
        };
    };
    Ok(())
}