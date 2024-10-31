use crate::repository::sdtd::request_sdtd_start;
use crate::{Context, Error};

/// Control 7 Days to Die server
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn sdtd(
    ctx: Context<'_>,
    #[description = "start, status, stop, restart"] command: String,
) -> Result<(), Error> {
    match command.as_str() {
        "start" => {
            ctx.say("Starting 7 Days to Die server").await?;
            request_sdtd_start().await?;
        }
        "status" => {
            ctx.say("Checking 7 Days to Die server status").await?;
            // repository::request_sdtd_status().await?;
        }
        "stop" => {
            ctx.say("Stopping 7 Days to Die server").await?;
            // repository::request_sdtd_stop().await?;
        }
        "restart" => {
            ctx.say("Restarting 7 Days to Die server").await?;
            // repository::request_sdtd_restart().await?;
        }
        _ => {
            ctx.say("Invalid command").await?;
        }
    }

    Ok(())
}
