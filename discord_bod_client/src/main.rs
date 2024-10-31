#![warn(clippy::str_to_string)]

use discord_bod_client::config::CONFIG;
use discord_bod_client::{poise_framework_options, setup_framework};
use poise::serenity_prelude as serenity;

#[tokio::main]
async fn main() {
    easy_init_newrelic_opentelemetry::NewRelicSubscriberInitializer::default()
        .newrelic_license_key(&CONFIG.newrelic_license_key)
        .newrelic_service_name(&CONFIG.newrelic_service_name)
        .host_name("localhost")
        .init()
        .expect("Failed to initialize New Relic OpenTelemetry");

    let options = poise_framework_options();
    let framework = setup_framework(options);

    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let client = serenity::ClientBuilder::new(&CONFIG.discord_token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap()
}

// FrameworkOptions contains all of poise's configuration option in one struct
// Every option can be omitted to use its default value
// let options = poise::FrameworkOptions {
//     commands: vec![
//         commands::help(),
//         commands::vote(),
//         commands::getvotes(),
//         commands::sdtd(),
//         commands::register(),
//     ],
//     prefix_options: poise::PrefixFrameworkOptions {
//         prefix: Some("~".into()),
//         edit_tracker: Some(Arc::new(poise::EditTracker::for_timespan(
//             Duration::from_secs(3600),
//         ))),
//         additional_prefixes: vec![
//             poise::Prefix::Literal("hey bot"),
//             poise::Prefix::Literal("hey bot,"),
//         ],
//         ..Default::default()
//     },
//     // The global error handler for all error cases that may occur
//     on_error: |error| Box::pin(on_error(error)),
//     // This code is run before every command
//     pre_command: |ctx| {
//         Box::pin(async move {
//             println!("Executing command {}...", ctx.command().qualified_name);
//         })
//     },
//     // This code is run after a command if it was successful (returned Ok)
//     post_command: |ctx| {
//         Box::pin(async move {
//             println!("Executed command {}!", ctx.command().qualified_name);
//         })
//     },
//     // Every command invocation must pass this check to continue execution
//     command_check: Some(|ctx| {
//         Box::pin(async move {
//             if ctx.author().id == 123456789 {
//                 return Ok(false);
//             }
//             Ok(true)
//         })
//     }),
//     // Enforce command checks even for owners (enforced by default)
//     // Set to true to bypass checks, which is useful for testing
//     skip_checks_for_owners: false,
//     event_handler: |_ctx, event, _framework, _data| {
//         Box::pin(async move {
//             println!(
//                 "Got an event in event handler: {:?}",
//                 event.snake_case_name()
//             );
//             Ok(())
//         })
//     },
//     ..Default::default()
// };
//
// let framework = poise::Framework::builder()
//     .setup(move |ctx, _ready, framework| {
//         Box::pin(async move {
//             println!("Logged in as {}", _ready.user.name);
//             poise::builtins::register_globally(ctx, &framework.options().commands).await?;
//             Ok(Data {
//                 votes: Mutex::new(HashMap::new()),
//             })
//         })
//     })
//     .options(options)
//     .build();
//
// let token = var("DISCORD_TOKEN")
//     .expect("Missing `DISCORD_TOKEN` env var, see README for more information.");
// let intents =
//     serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;
//
// let client = serenity::ClientBuilder::new(token, intents)
//     .framework(framework)
//     .await;
//
// client.unwrap().start().await.unwrap()
