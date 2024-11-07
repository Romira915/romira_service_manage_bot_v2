use serde::{Deserialize, Serialize};
use std::fmt::Display;

pub mod sdtd;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum SystemdCommand {
    Start,
    Stop,
    Restart,
    #[default]
    IsActive,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum SystemdStatus {
    Active,
    #[default]
    Inactive,
}

impl Display for SystemdStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SystemdStatus::Active => write!(f, "active"),
            SystemdStatus::Inactive => write!(f, "inactive"),
        }
    }
}
