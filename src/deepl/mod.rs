use crate::deepl::translate::{DeeplAnswer,Language};

mod translate;


pub struct DeeplClient {
    url:String,
    key:String,
    client:reqwest::Client
}

impl DeeplClient {
    pub fn new(key: &str) -> Self {
        DeeplClient {
            url: "https://api-free.deepl.com/v2/translate".to_string(),
            key: key.to_string(),
            client:reqwest::Client::new()
        }
    }

    pub fn with_client(key: &str, client: &reqwest::Client) -> Self {
        DeeplClient {
            url: "https://api-free.deepl.com/v2/translate".to_string(),
            key: key.to_string(),
            client:client.clone()
        }
    }

    pub async fn translate(self, text:String, target_lang:Language) -> Result<DeeplAnswer,reqwest::Error>{
        return self.client.post(self.url)
            .header("Authorization",format!("DeepL-Auth-Key {}", self.key.as_str()))
            .form(&[("text",text),("target_lang",target_lang.to_string())])
            .send()
            .await?
            .json::<DeeplAnswer>()
            .await;
    }


}