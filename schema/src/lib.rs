use serde::{Deserialize, Serialize};

pub mod sdtd;

#[derive(Debug, Default, Serialize, Deserialize)]
pub enum SystemdCommand {
    Start,
    Stop,
    Restart,
    #[default]
    IsActive,
}
