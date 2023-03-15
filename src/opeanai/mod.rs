mod chat;
mod completion;
mod edit;
mod structs;

use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub struct OpenAiClient {
    url:String,
    key:String,
    client:reqwest::Client
}

impl OpenAiClient {

    const URL: &'static str = "https://api.openai.com/v1";

    pub fn new(key: &str)->Self{
        OpenAiClient {
            url: OpenAiClient::URL.to_string(),
            key: key.to_string(),
            client:reqwest::Client::new()
        }
    }

    /// reqwest library recommends reusing single client,
    /// so if you run access to multiple api-s, pass client into constructor
    pub fn with_client(key: &str, client: &reqwest::Client)->Self{
        OpenAiClient {
            url: OpenAiClient::URL.to_string(),
            key: key.to_string(),
            client: client.clone()
        }
    }

}

#[async_trait(?Send)]
trait PostClient<TReq: Serialize + Sized,TRes: DeserializeOwned>{

    const ENDPOINT: &'static str;

    fn get_client(&self)->reqwest::Client;
    fn get_key(&self)->&str;
    fn get_url(&self)->&str;

    async fn run(&self, req:TReq)-> Result<TRes,reqwest::Error>{
        let final_url = self.get_url().to_owned()+Self::ENDPOINT;
        return self.get_client().post(final_url)
            .bearer_auth(self.get_key())
            .json(&req)
            .send()
            .await?
            .json::<TRes>()
            .await;
    }
}
