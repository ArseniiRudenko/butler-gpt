use crate::deepl::translate::{DeeplAnswer,Language};

mod translate;


pub struct DeeplClient {
    url:String,
    key:String,
    client:reqwest::Client
}

impl DeeplClient {

    const URL: &'static str = "https://api-free.deepl.com/v2/translate";

    pub fn new(key: &str) -> Self {
        let client = reqwest::Client::new();
        return DeeplClient::with_client(key,&client)
    }

    pub fn with_client(key: &str, client: &reqwest::Client) -> Self {
        return DeeplClient ::with_url_and_client(key,DeeplClient::URL,client)
    }

    pub fn with_url(key: &str, url: &str) -> Self {
        let client = reqwest::Client::new();
        return  DeeplClient ::with_url_and_client(key,url,&client)
    }

    pub fn with_url_and_client(key: &str, url: &str, client: &reqwest::Client) -> Self {
        DeeplClient {
            url: url.to_string(),
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