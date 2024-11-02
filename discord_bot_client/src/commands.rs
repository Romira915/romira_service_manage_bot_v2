use crate::{Context, Error};

pub(crate) mod sdtd;

/// Show this help menu
#[poise::command(prefix_command, track_edits, slash_command)]
pub(crate) async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), Error> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            extra_text_at_bottom: "~ Welcome to ServerManage Help! ~",
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}

#[poise::command(prefix_command, hide_in_help)]
pub async fn register(ctx: Context<'_>, #[flag] global: bool) -> Result<(), Error> {
    poise::builtins::register_application_commands(ctx, global).await?;
    Ok(())
}
