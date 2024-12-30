use poise::serenity_prelude as serenity;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tracing::log::debug;

pub(crate) mod commands;
pub mod config;
pub(crate) mod error;
pub(crate) mod repository;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data;

pub fn poise_framework_options() -> poise::FrameworkOptions<Data, Error> {
    poise::FrameworkOptions {
        commands: vec![
            commands::help(),
            commands::register(),
            commands::sdtd::sdtd(),
            commands::wol::wol(),
        ],
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("~".into()),
            edit_tracker: Some(Arc::new(poise::EditTracker::for_timespan(
                Duration::from_secs(3600),
            ))),
            additional_prefixes: vec![
                poise::Prefix::Literal("hey bot"),
                poise::Prefix::Literal("hey bot,"),
            ],
            ..Default::default()
        },
        on_error: |error| Box::pin(error::on_error(error)),
        skip_checks_for_owners: false,
        event_handler: |_ctx, event: &serenity::FullEvent, _framework, _data| {
            Box::pin(async move {
                tracing::debug!(
                    "Got an event in event handler: {:?}",
                    event.snake_case_name()
                );
                Ok(())
            })
        },
        ..Default::default()
    }
}

pub fn setup_framework(
    options: poise::FrameworkOptions<Data, Error>,
) -> poise::Framework<Data, Error> {
    poise::Framework::builder()
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                tracing::debug!("Logged in as {}", _ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data)
            })
        })
        .options(options)
        .build()
}
