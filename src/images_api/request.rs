use serde::Serialize;

#[derive(Debug, Clone, Default)]
pub struct RequestBuilder {
    /// A text description of the desired image(s). The maximum length is 1000 characters for `dall-e-2` and 4000 characters for `dall-e-3`.
    pub prompt: Option<String>,
    /// The model to use for image generation.
    pub model: Option<Model>,
    /// The number of images to generate. Must be between 1 and 10. For `dall-e-3`, only `n=1` is supported.
    pub n: Option<i32>,
    /// The quality of the image that will be generated.
    ///
    /// `hd` creates images with finer details and greater consistency across the image. This param is only supported for `dall-e-3`.
    pub quality: Option<Quality>,
    /// The format in which the generated images are returned.
    /// 
    /// Must be one of `url` or `b64_json`.
    /// URLs are only valid for 60 minutes after the image has been generated.
    pub response_format: Option<ResponseFormat>,
    /// The size of the generated images.
    /// 
    /// * Must be one of `256x256`, `512x512`, or `1024x1024` for `dall-e-2`.
    /// * Must be one of `1024x1024`, `1792x1024`, or `1024x1792` for `dall-e-3` models.
    pub size: Option<Size>,
    /// A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse.
    pub user: Option<String>,
}

impl RequestBuilder {
    /// A text description of the desired image(s). The maximum length is 1000 characters for `dall-e-2` and 4000 characters for `dall-e-3`.
    pub fn with_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.prompt = Some(prompt.into());
        self
    }
    /// The model to use for image generation.
    pub fn with_model(mut self, model: impl Into<Model>) -> Self {
        self.model = Some(model.into());
        self
    }
    /// The number of images to generate. Must be between 1 and 10. For `dall-e-3`, only `n=1` is supported.
    pub fn with_n(mut self, n: i32) -> Self {
        self.n = Some(n);
        self
    }
    /// The quality of the image that will be generated.
    ///
    /// `hd` creates images with finer details and greater consistency across the image. This param is only supported for `dall-e-3`.
    pub fn with_quality(mut self, quality: impl Into<Quality>) -> Self {
        self.quality = Some(quality.into());
        self
    }
    /// The format in which the generated images are returned.
    /// 
    /// Must be one of `url` or `b64_json`.
    /// URLs are only valid for 60 minutes after the image has been generated.
    pub fn with_response_format(mut self, response_format: impl Into<ResponseFormat>) -> Self {
        self.response_format = Some(response_format.into());
        self
    }
    /// The size of the generated images.
    /// 
    /// * Must be one of `256x256`, `512x512`, or `1024x1024` for `dall-e-2`.
    /// * Must be one of `1024x1024`, `1792x1024`, or `1024x1792` for `dall-e-3` models.
    pub fn with_size(mut self, size: impl Into<Size>) -> Self {
        self.size = Some(size.into());
        self
    }
    /// A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse.
    pub fn with_user(mut self, user: impl Into<String>) -> Self {
        self.user = Some(user.into());
        self
    }
    pub fn build(self) -> Option<Request> {
        Some(Request {
            prompt: self.prompt?,
            model: self.model.map(|x| x.0),
            n: self.n,
            quality: self.quality.map(|x| x.0),
            response_format: self.response_format.map(|x| x.0),
            size: self.size.map(|x| x.0),
            user: self.user,
        })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    /// A text description of the desired image(s). The maximum length is 1000 characters for `dall-e-2` and 4000 characters for `dall-e-3`.
    pub prompt: String,
    /// The model to use for image generation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// The number of images to generate. Must be between 1 and 10. For `dall-e-3`, only `n=1` is supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<i32>,
    /// The quality of the image that will be generated.
    ///
    /// `hd` creates images with finer details and greater consistency across the image. This param is only supported for `dall-e-3`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<String>,
    /// The format in which the generated images are returned.
    /// 
    /// Must be one of `url` or `b64_json`.
    /// URLs are only valid for 60 minutes after the image has been generated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<String>,
    /// The size of the generated images.
    /// 
    /// * Must be one of `256x256`, `512x512`, or `1024x1024` for `dall-e-2`.
    /// * Must be one of `1024x1024`, `1792x1024`, or `1024x1792` for `dall-e-3` models.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    /// A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(transparent)]
pub struct Model(pub String);

impl Model {
    pub fn dall_e2() -> Self {
        Self(String::from("dall-e-2"))
    }
    pub fn dall_e3() -> Self {
        Self(String::from("dall-e-3"))
    }
}

/// The quality of the image that will be generated.
///
/// `hd` creates images with finer details and greater consistency across the image. This param is only supported for `dall-e-3`.
#[derive(Debug, Clone, Serialize)]
#[serde(transparent)]
pub struct Quality(pub String);

impl Quality {
    pub fn standard() -> Self {
        Self(String::from("standard"))
    }
    pub fn hd() -> Self {
        Self(String::from("hd"))
    }
}

/// The format in which the generated images are returned.
/// 
/// Must be one of `url` or `b64_json`.
/// URLs are only valid for 60 minutes after the image has been generated.
#[derive(Debug, Clone, Serialize)]
#[serde(transparent)]
pub struct ResponseFormat(pub String);

impl ResponseFormat {
    pub fn url() -> Self {
        Self(String::from("url"))
    }
    pub fn b64_json() -> Self {
        Self(String::from("b64_json"))
    }
}

/// The size of the generated images.
/// 
/// * Must be one of `256x256`, `512x512`, or `1024x1024` for `dall-e-2`.
/// * Must be one of `1024x1024`, `1792x1024`, or `1024x1792` for `dall-e-3` models.
#[derive(Debug, Clone, Serialize)]
#[serde(transparent)]
pub struct Size(pub String);

impl Size {
    pub fn dall_e2_256x256() -> Self {
        Self(String::from("256x256"))
    }
    pub fn dall_e2_512x512() -> Self {
        Self(String::from("512x512"))
    }
    pub fn dall_e2_1024x1024() -> Self {
        Self(String::from("1024x1024"))
    }
    pub fn dall_e3_1024x1024() -> Self {
        Self(String::from("1024x1024"))
    }
    pub fn dall_e3_1792x1024() -> Self {
        Self(String::from("1792x1024"))
    }
    pub fn dall_e3_1024x1792() -> Self {
        Self(String::from("1024x1792"))
    }
}

impl From<String> for Model {
    fn from(value: String) -> Self { Self(value) }
}
impl From<String> for Quality {
    fn from(value: String) -> Self { Self(value) }
}
impl From<String> for ResponseFormat {
    fn from(value: String) -> Self { Self(value) }
}
impl From<String> for Size {
    fn from(value: String) -> Self { Self(value) }
}

impl From<&str> for Model {
    fn from(value: &str) -> Self { Self(value.to_string()) }
}
impl From<&str> for Quality {
    fn from(value: &str) -> Self { Self(value.to_string()) }
}
impl From<&str> for ResponseFormat {
    fn from(value: &str) -> Self { Self(value.to_string()) }
}
impl From<&str> for Size {
    fn from(value: &str) -> Self { Self(value.to_string()) }
}