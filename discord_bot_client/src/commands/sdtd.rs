use crate::repository::sdtd::{
    request_sdtd_restart, request_sdtd_start, request_sdtd_status, request_sdtd_stop,
};
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
            ctx.say("7 Days to Die server started").await?;
        }
        "status" => {
            ctx.say("Checking 7 Days to Die server status").await?;
            let status = request_sdtd_status().await?;
            ctx.say(format!("7 Days to Die server status: {}", status))
                .await?;
        }
        "stop" => {
            ctx.say("Stopping 7 Days to Die server").await?;
            request_sdtd_stop().await?;
            ctx.say("7 Days to Die server stopped").await?;
        }
        "restart" => {
            ctx.say("Restarting 7 Days to Die server").await?;
            request_sdtd_restart().await?;
            ctx.say("7 Days to Die server restarted").await?;
        }
        _ => {
            ctx.say("Invalid command").await?;
        }
    }

    Ok(())
}
