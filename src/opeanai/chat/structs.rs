use std::collections::HashMap;
use strum::Display;
use strum::EnumString;
use serde::{Serialize,Deserialize};

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
pub struct ChatRequest {
    model:String,
    messages:Vec<Message>,
    temperature: Option<f64>,
    top_p: Option<f64>,
    n: Option<u16>,
    stream: Option<bool>,
    stop:Option<StopSeq>,
    max_tokens: Option<u64>,
    presence_penalty: Option<f64>,
    frequency_penalty:Option<f64>,
    logit_bias: Option<HashMap<i32,i32>>,
    user: Option<String>
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
pub struct ChatResponse {
    id: String,
    object: String,
    created: u64,
    choices: Vec<Choice>,
    usage:Usage
}


impl ChatRequest {

    pub fn new(messages : Vec<Message>) -> Self {
        Self {
            model: "gpt-3.5-turbo".to_string(),
            messages,
            temperature: None,
            top_p: None,
            n: None,
            stream: None,
            stop: None,
            max_tokens: None,
            presence_penalty: None,
            frequency_penalty: None,
            logit_bias: None,
            user: None,
        }
    }

    pub fn with_model_and_messages(model: &str, messages : Vec<Message>) -> Self {
        Self {
            model: model.to_string(),
            messages,
            temperature: None,
            top_p: None,
            n: None,
            stream: None,
            stop: None,
            max_tokens: None,
            presence_penalty: None,
            frequency_penalty: None,
            logit_bias: None,
            user: None,
        }
    }

    pub fn add_message(mut self, message:Message) ->Self{
        self.messages.push(message);
        self
    }

    pub fn model(mut self, model: String) -> Self {
        self.model = model;
        self
    }

    pub fn temperature(mut self, temperature: f64) -> Self {
        if self.top_p.is_some() {
            self.top_p = None;
        }
        self.temperature = Some(temperature.clamp(0f64,2f64));
        self
    }

    pub fn top_p(mut self, top_p: f64) -> Self {
        if self.temperature.is_some() {
            self.temperature = None;
        }
        self.top_p = Some(top_p.clamp(0f64,1f64));
        self
    }

    pub fn n(mut self, n: u16) -> Self {
        self.n = Some(n);
        self
    }

    pub fn stream(mut self, stream: bool) -> Self {
        self.stream = Some(stream);
        self
    }

    pub fn stop(mut self, stop: StopSeq) -> Self {
        self.stop = Some(stop);
        self
    }

    pub fn max_tokens(mut self, max_tokens: u64) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    pub fn presence_penalty(mut self, presence_penalty: f64) -> Self{
        self.presence_penalty= Some(presence_penalty.clamp(-2f64,2f64));
        self
    }

    pub fn frequency_penalty(mut self, frequency_penalty: f64) -> Self {
        self.frequency_penalty = Some(frequency_penalty.clamp(-2f64,2f64));
        self
    }

    pub fn logit_bias(mut self, logit_bias: HashMap<i32, i32>) -> Self {
        self.logit_bias = Some(logit_bias);
        self
    }

    pub fn user(mut self, user: String) -> Self {
        self.user = Some(user);
        self
    }

}
