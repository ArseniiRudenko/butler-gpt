use std::collections::HashMap;
use strum::Display;
use strum::EnumString;
use serde::{Deserialize, Serialize};


#[derive(Debug, Display, EnumString, Eq, PartialEq,Serialize,Deserialize)]
#[strum(serialize_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
}

#[derive(Serialize, Deserialize)]
pub struct Message{
    role:Role,
    content:String
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum StopSeq{
    String(String),
    Vec(Vec<String>)
}

///
/// # Example
/// ```
/// {
///   "model": "gpt-3.5-turbo",
///   "messages": [{"role": "user", "content": "Hello!"}]
/// }
/// ```
///
#[derive(Serialize, Deserialize)]
pub struct Request{
    model:String,
    messages:Vec<Message>,
    temperature: Option<f64>,
    top_p: Option<f64>,
    n: Option<u16>,
    stream: Option<bool>,
    stop:Option<StopSeq>,
    max_tokens: Option<u64>,
    presence_penalty: Option<f64>,
    frequency_penalty:Option<u64>,
    logit_bias: Option<HashMap<i32,i32>>,
    user: Option<String>
}

impl Default for Request {
    fn default() -> Self {
        Request{
            model: "gpt-3.5-turbo".to_string(),
            messages: vec!(),
            temperature: None,
            top_p: None,
            n: None,
            stream: None,
            stop:None,
            max_tokens: None,
            presence_penalty: None,
            frequency_penalty:None,
            logit_bias: None,
            user: None
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Choice{
    index: u16,
    message: Message,
    finish_reason: String
}

#[derive(Serialize, Deserialize)]
pub struct Usage{
    prompt_tokens: u64,
    completion_tokens: u64,
    total_tokens: u64
}

/// Chat endpoint response
///
/// # Example
/// ```
/// {
///   "id": "chatcmpl-123",
///   "object": "chat.completion",
///   "created": 1677652288,
///   "choices": [{
///     "index": 0,
///     "message": {
///       "role": "assistant",
///       "content": "\n\nHello there, how may I assist you today?",
///     },
///     "finish_reason": "stop"
///   }],
///   "usage": {
///     "prompt_tokens": 9,
///     "completion_tokens": 12,
///     "total_tokens": 21
///   }
/// }
/// ```
///
#[derive(Serialize, Deserialize)]
pub struct Response{
    id: String,
    object: String,
    created: u64,
    choices: Vec<Choice>,
    usage:Usage
}




