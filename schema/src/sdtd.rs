use crate::SystemdCommand;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SdtdRequestJson {
    pub command: SystemdCommand,
}
