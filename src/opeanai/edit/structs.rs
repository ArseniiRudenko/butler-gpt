use std::collections::HashMap;

use serde::{Serialize,Deserialize};
use crate::opeanai::structs::Usage;


#[derive(Serialize, Deserialize, Debug)]
pub struct EditRequest {
    model: String,
    input: Option<String>,
    instruction: String,
    n: Option<u16>,
    temperature: Option<f32>,
    top_p: Option<f32>,
}

#[derive(Serialize, Deserialize)]
pub struct EditChoice {
    pub text: String,
    pub index: i64,
}

#[derive(Serialize, Deserialize)]
pub struct EditResponse {
    pub object: String,
    pub created: i64,
    pub choices: Vec<EditChoice>,
    pub usage: Usage,
}

impl EditRequest {

    pub fn new_text(instruction: &str) -> Self {
        Self {
            model: "text-davinci-edit-001".to_string(),
            input: None,
            instruction: instruction.to_string(),
            n: None,
            temperature: None,
            top_p: None,
        }
    }

    pub fn new_code(instruction: &str) -> Self {
        Self {
            model: "code-davinci-edit-001".to_string(),
            input: None,
            instruction: instruction.to_string(),
            n: None,
            temperature: None,
            top_p: None,
        }
    }

    pub fn with_model(model: &str, instruction: &str) -> Self {
        Self {
            model: model.to_string(),
            input: None,
            instruction: instruction.to_string(),
            n: None,
            temperature: None,
            top_p: None,
        }
    }

    pub fn set_input(mut self, input: String) -> Self {
        self.input = Some(input);
        self
    }

    pub fn set_n(mut self, n: u16) -> Self {
        self.n = Some(n);
        self
    }

    pub fn set_temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    pub fn set_top_p(mut self, top_p: f32) -> Self {
        self.top_p = Some(top_p);
        self
    }
}
