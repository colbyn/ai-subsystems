pub use crate::text_api::client::ApiError;

#[derive(Default)]
pub struct ClientConfigurationBuilder {
    pub api_url: Option<URL>,
    pub api_key: Option<ApiKey>,
    pub timeout: Option<Timeout>,
}

impl ClientConfigurationBuilder {
    pub fn with_api_url(mut self, api_url: impl Into<URL>) -> Self {
        self.api_url = Some(api_url.into());
        self
    }
    pub fn with_api_key(mut self, api_key: impl Into<ApiKey>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }
    pub fn with_timeout(mut self, timeout: impl Into<Timeout>) -> Self {
        self.timeout = Some(timeout.into());
        self
    }
    pub fn build(self) -> Option<ClientConfiguration> {
        Some(ClientConfiguration {
            api_url: self.api_url?,
            api_key: self.api_key?,
            timeout: self.timeout
        })
    }
}

pub struct ClientConfiguration {
    pub api_url: URL,
    pub api_key: ApiKey,
    pub timeout: Option<Timeout>,
}

pub struct URL(pub String);

impl URL {
    /// Creates an image given a prompt.
    pub fn openai_v1_images_generations() -> Self {
        Self(String::from("https://api.openai.com/v1/images/generations"))
    }
}

pub struct ApiKey(pub String);

pub struct Timeout(pub std::time::Duration);

impl From<String> for URL {
    fn from(value: String) -> Self { Self(value) }
}
impl From<String> for ApiKey {
    fn from(value: String) -> Self { Self(value) }
}
impl From<&str> for URL {
    fn from(value: &str) -> Self { Self(value.to_string()) }
}
impl From<&str> for ApiKey {
    fn from(value: &str) -> Self { Self(value.to_string()) }
}
impl From<std::time::Duration> for Timeout {
    fn from(value: std::time::Duration) -> Self { Self(value) }
}

impl super::request::Request {
    pub async fn execute(
        self,
        client_configuration: &ClientConfiguration
    ) -> Result<super::response::Response, Box<dyn std::error::Error>> {
        let api_url = client_configuration.api_url.0.as_str();
        let api_key = client_configuration.api_key.0.as_str();
        let client = {
            if let Some(timeout) = client_configuration.timeout.as_ref() {
                reqwest::ClientBuilder::new()
                    .timeout(timeout.0.clone())
                    .build()
                    .unwrap()
            } else {
                reqwest::ClientBuilder::new().build().unwrap()
            }
        };
        let json_data = serde_json::to_string(&self).unwrap();
        let response = client
            .post(api_url)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            // .json(&self.client.request_body)
            .body(json_data)
            .send()
            .await?;
        if let Some(error) = ApiError::from_code(response.status().as_u16()) {
            return Err(Box::new(error))
        }
        let result = response.text().await?;
        let result = serde_json::from_str::<super::response::Response>(&result)?;
        Ok(result)
    }
}