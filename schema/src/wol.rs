use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum WolTarget {
    Amd3900X,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WolRequestJson {
    pub target: WolTarget,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WolResponseJson {
    pub result: String,
}
