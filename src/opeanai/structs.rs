use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Usage{
    prompt_tokens: u64,
    completion_tokens: u64,
    total_tokens: u64
}