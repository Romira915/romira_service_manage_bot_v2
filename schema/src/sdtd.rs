use crate::{SystemdCommand, SystemdStatus};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SdtdRequestJson {
    pub command: SystemdCommand,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SdtdResponseJson<'a> {
    pub result: &'a str,
    pub status: Option<SystemdStatus>,
}

impl<'a> SdtdResponseJson<'a> {
    pub fn new(result: &'a str, status: SystemdStatus) -> Self {
        Self {
            result,
            status: Some(status),
        }
    }

    pub fn from_result(result: &'a str) -> Self {
        Self {
            result,
            ..Default::default()
        }
    }
}
