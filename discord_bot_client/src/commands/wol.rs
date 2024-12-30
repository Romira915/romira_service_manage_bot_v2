use crate::repository::wol::request_wol;
use crate::{Context, Error};
use schema::wol::WolTarget;

/// Send Wake-on-LAN magic packet
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn wol(
    ctx: Context<'_>,
    #[description = "Send Wake-on-LAN magic packet"] command: String,
) -> Result<(), Error> {
    if ctx.author().id != 515120152749146112 {
        ctx.say("You are not allowed to use this command").await?;
        return Ok(());
    }

    match command.as_str().to_lowercase().as_str() {
        "amd3900x" => {
            ctx.say("Sending Wake-on-LAN magic packet to AMD3900X")
                .await?;
            request_wol(WolTarget::Amd3900X).await?;
            ctx.say("Wake-on-LAN magic packet sent to AMD3900X").await?;
        }
        _ => {
            ctx.say("Invalid command").await?;
        }
    }

    Ok(())
}
