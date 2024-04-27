#[derive(Debug, Clone, serde::Deserialize)]
pub struct Response {
    pub created: isize,
    pub data: Vec<Image>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Image {
    pub b64_json: Option<B64>,
    pub url: Option<String>,
    pub revised_prompt: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(transparent)]
pub struct B64(pub String);

impl B64 {
    pub fn decode(&self) -> Result<Vec<u8>, base64::DecodeError> {
        use base64::prelude::*;
        BASE64_STANDARD.decode(&self.0)
    }
}