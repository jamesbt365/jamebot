#![warn(clippy::pedantic)]
#![allow(clippy::wildcard_imports)]

mod commands;
use commands::*;
mod event_handler;
use poise::serenity_prelude as serenity;
use std::{env::var, sync::Arc, time::Duration};

use jamebot_data::{Command, Context, Data, Error};

use std::borrow::Cow;

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        poise::FrameworkError::Command { error, ctx, .. } => {
            println!("Error in command `{}`: {:?}", ctx.command().name, error);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {e}");
            }
        }
    }
}

async fn get_prefix(
    ctx: jamebot_data::PartialContext<'_>,
) -> Result<Option<Cow<'static, str>>, Error> {
    // If not in a guild, return the default prefix.
    let Some(guild_id) = ctx.guild_id else {
        return Ok(Some(Cow::Borrowed("-")));
    };

    let data = ctx.framework.user_data();

    let config = data.get_guild(guild_id).await;

    if let Some(prefix) = config.prefix {
        let prefix = if prefix == "-" {
            Cow::Borrowed("-")
        } else {
            Cow::Owned(prefix)
        };

        Ok(Some(prefix))
    } else {
        Ok(Some(Cow::Borrowed("-")))
    }
}

#[tokio::main]
async fn main() {
    let options = poise::FrameworkOptions {
        commands: commands(),
        prefix_options: poise::PrefixFrameworkOptions {
            dynamic_prefix: Some(|ctx| Box::pin(get_prefix(ctx))),
            edit_tracker: Some(Arc::new(poise::EditTracker::for_timespan(
                Duration::from_secs(3600),
            ))),
            ..Default::default()
        },

        on_error: |error| Box::pin(on_error(error)),

        skip_checks_for_owners: false,
        event_handler: |framework, event| Box::pin(event_handler::event_handler(framework, event)),
        ..Default::default()
    };

    let framework = poise::Framework::new(options);

    // eventually only use the intents I need.
    let intents = serenity::GatewayIntents::non_privileged()
        | serenity::GatewayIntents::MESSAGE_CONTENT
        | serenity::GatewayIntents::GUILD_MEMBERS
        | serenity::GatewayIntents::GUILD_PRESENCES;

    let token = var("JAMEBOT_TOKEN").expect("JAMEBOT_TOKEN is not set.");

    let mut client = serenity::Client::builder(&token, intents)
        .framework(framework)
        .data(Data::new().await)
        .await
        .unwrap();

    client.start().await.unwrap();
}
