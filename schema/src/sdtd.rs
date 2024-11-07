use crate::{SystemdCommand, SystemdStatus};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SdtdRequestJson {
    pub command: SystemdCommand,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SdtdResponseJson {
    pub result: String,
    pub status: Option<SystemdStatus>,
}

impl SdtdResponseJson {
    pub fn new(result: impl ToString, status: SystemdStatus) -> Self {
        Self {
            result: result.to_string(),
            status: Some(status),
        }
    }

    pub fn from_result(result: impl ToString) -> Self {
        Self {
            result: result.to_string(),
            ..Default::default()
        }
    }
}
