use crate::opeanai::chat::structs::{Request, Response};

pub struct ChatClient {
    url:String,
    key:String,
    client:reqwest::Client
}

impl ChatClient {

    const URL: &'static str = "https://api.openai.com/v1/chat/completions";

    pub fn new(key: &str)->Self{
        ChatClient {
            url: ChatClient::URL.to_string(),
            key: key.to_string(),
            client:reqwest::Client::new()
        }
    }

    /// reqwest library recommends reusing single client,
    /// so if you run access to multiple api-s, pass client into constructor
    pub fn with_client(key: &str, client: &reqwest::Client)->Self{
        ChatClient {
            url: ChatClient::URL.to_string(),
            key: key.to_string(),
            client: client.clone()
        }
    }

    pub async fn run(self, req:Request)-> Result<Response,reqwest::Error> {
        return self.client.post(self.url)
            .bearer_auth(self.key.as_str())
            .json(&req)
            .send()
            .await?
            .json::<Response>()
            .await;
    }

}
