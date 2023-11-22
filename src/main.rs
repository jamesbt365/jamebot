mod commands;
use commands::*;
mod config;
mod event_handler;

use config::GuildConfig;
use poise::serenity_prelude::{self as serenity, GuildId};
use std::{env::var, time::Duration};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

use sqlx::{postgres::PgPoolOptions, PgPool};

pub struct Data {
    database: PgPool,
    guild_cache: DashMap<GuildId, GuildConfig>,
    time_started: std::time::Instant,
}

#[poise::command(prefix_command, hide_in_help)]
async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;

    Ok(())
}

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx, .. } => {
            println!("Error in command `{}`: {:?}", ctx.command().name, error,);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {}", e)
            }
        }
    }
}

use dashmap::DashMap;

#[tokio::main]
async fn main() {
    let database = {
        let database_url =
            std::env::var("DATABASE_URL").expect("No database url found in environment variables!");

        let database = PgPoolOptions::new()
            .connect(&database_url)
            .await
            .expect("Failed to connect to database!");

        sqlx::migrate!()
            .run(&database)
            .await
            .expect("Unable to apply migrations!");

        database
    };

    let options = poise::FrameworkOptions {
        commands: vec![
            register(),
            meta::source(),
            meta::shutdown(),
            meta::uptime(),
            meta::help(),
            fun::hug::hug(),
            utility::roll::roll(),
            general::avatar::avatar(),
            utility::colour::hex(),
        ],
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: None,
            dynamic_prefix: Some(|ctx| {
                Box::pin(async move {
                    match ctx.guild_id {
                        Some(guild_id) => {
                            // generate guild config from
                            let guild_config = ctx.data.guild_cache.get(&guild_id);
                            let prefix = if let Some(config) = guild_config {
                                config.prefix.clone()
                            } else {
                                // No guild config, add one and use default config.
                                // Maybe do a check for the guild first, then do it.
                                let config = config::add_guild_config_def(ctx.data, guild_id).await;
                                // gonna do a lazy unwrap here because I'm not sure how I would actually want to handle a problem.
                                config.unwrap().prefix
                            };
                            Ok(prefix)
                        }
                        None => {
                            // No guild, use default prefix.
                            Ok(Some(String::from("-")))
                        }
                    }
                })
            }),
            edit_tracker: Some(poise::EditTracker::for_timespan(Duration::from_secs(3600))),
            ..Default::default()
        },

        on_error: |error| Box::pin(on_error(error)),

        pre_command: |ctx| {
            Box::pin(async move {
                println!("Executing command {}...", ctx.command().qualified_name);
            })
        },

        post_command: |ctx| {
            Box::pin(async move {
                println!("Executed command {}!", ctx.command().qualified_name);
            })
        },

        skip_checks_for_owners: false,
        event_handler: |event: &serenity::FullEvent, framework, data| {
            Box::pin(event_handler::event_handler(event.clone(), framework, data))
        },
        ..Default::default()
    };

    let framework = poise::Framework::new(options, move |ctx, ready, framework| {
        Box::pin(async move {
            println!("Logged in as {}", ready.user.name);
            poise::builtins::register_globally(ctx, &framework.options().commands).await?;
            Ok(Data {
                database,
                guild_cache: DashMap::new(),
                time_started: std::time::Instant::now(),
            })
        })
    });

    let intents = serenity::GatewayIntents::non_privileged()
        | serenity::GatewayIntents::MESSAGE_CONTENT
        | serenity::GatewayIntents::GUILD_MEMBERS
        | serenity::GatewayIntents::GUILD_PRESENCES;

    let token = var("JAMEBOT_TOKEN").expect("JAMEBOT_TOKEN is not set.");

    let mut client = serenity::Client::builder(token, intents)
        .framework(framework)
        .await
        .unwrap();

    client.start().await.unwrap();
}
