use crate::SystemdControl;
use schema::SystemdCommand;

pub(crate) async fn sdtd_systemd_executable(
    sdtd_systemd: impl SystemdControl,
    command: SystemdCommand,
) -> anyhow::Result<bool> {
    match command {
        SystemdCommand::Start => {
            sdtd_systemd.start().await?;
        }
        SystemdCommand::Stop => {
            sdtd_systemd.stop().await?;
        }
        SystemdCommand::Restart => {
            sdtd_systemd.restart().await?;
        }
        SystemdCommand::IsActive => {
            let is_active = sdtd_systemd.is_active().await?;
            if !is_active {
                return Ok(false);
            }
        }
    }
    Ok(true)
}
