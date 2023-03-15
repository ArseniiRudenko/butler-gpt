use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Usage{
    prompt_tokens: u64,
    completion_tokens: u64,
    total_tokens: u64
}


#[derive(Serialize, Deserialize)]
pub struct FileInfo {
    pub id: String,
    pub object: String,
    pub bytes: i64,
    pub created_at: i64,
    pub filename: String,
    pub purpose: String,
}

#[derive(Serialize, Deserialize)]
pub struct FilesResponse {
    pub data: Vec<FileInfo>,
    pub object: String,
}