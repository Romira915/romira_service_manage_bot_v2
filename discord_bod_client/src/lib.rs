use std::collections::HashMap;
use std::sync::Mutex;

// Types used by all command functions
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

// Custom user data passed to all command functions
pub struct Data {
    pub votes: Mutex<HashMap<String, u32>>,
}

pub mod commands;
