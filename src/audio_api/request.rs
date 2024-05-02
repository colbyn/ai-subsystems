#[derive(Debug, Clone, Default)]
pub struct RequestBuilder {
    /// One of the available [TTS models](https://platform.openai.com/docs/models/tts): `tts-1` or `tts-1-hd`.
    pub model: Option<Model>,
    /// The text to generate audio for. The maximum length is 4096 characters.
    pub input: Option<String>,
    /// The voice to use when generating the audio. Supported voices are `alloy`, `echo`, `fable`, `onyx`, `nova`, and `shimmer`. Previews of the voices are available in the [Text to speech guide](https://platform.openai.com/docs/guides/text-to-speech/voice-options).
    pub voice: Option<Voice>,
    /// The format to audio in. Supported formats are `mp3`, `opus`, `aac`, `flac`, `wav`, and `pcm`.
    pub response_format: Option<ResponseFormat>,
    /// The speed of the generated audio. Select a value from `0.25` to `4.0`. `1.0` is the default.
    pub speed: Option<f32>,
}

impl RequestBuilder {
    pub fn with_model(mut self, model: impl Into<Model>) -> Self {
        self.model = Some(model.into());
        self
    }
    pub fn with_input(mut self, input: impl ToString) -> Self {
        self.input = Some(input.to_string());
        self
    }
    pub fn with_voice(mut self, voice: impl Into<Voice>) -> Self {
        self.voice = Some(voice.into());
        self
    }
    pub fn with_response_format(mut self, response_format: impl Into<ResponseFormat>) -> Self {
        self.response_format = Some(response_format.into());
        self
    }
    pub fn with_speed(mut self, speed: impl Into<f32>) -> Self {
        self.speed = Some(speed.into());
        self
    }
    pub fn build(self) -> Option<Request> {
        Some(Request {
            model: self.model?,
            input: self.input?,
            voice: self.voice?,
            response_format: self.response_format,
            speed: self.speed
        })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Request {
    /// One of the available [TTS models](https://platform.openai.com/docs/models/tts): `tts-1` or `tts-1-hd`.
    pub model: Model,
    /// The text to generate audio for. The maximum length is 4096 characters.
    pub input: String,
    /// The voice to use when generating the audio. Supported voices are `alloy`, `echo`, `fable`, `onyx`, `nova`, and `shimmer`. Previews of the voices are available in the [Text to speech guide](https://platform.openai.com/docs/guides/text-to-speech/voice-options).
    pub voice: Voice,
    /// The format to audio in. Supported formats are `mp3`, `opus`, `aac`, `flac`, `wav`, and `pcm`.
    pub response_format: Option<ResponseFormat>,
    /// The speed of the generated audio. Select a value from `0.25` to `4.0`. `1.0` is the default.
    pub speed: Option<f32>,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(transparent)]
pub struct Model(pub String);

#[derive(Debug, Clone, serde::Serialize)]
#[serde(transparent)]
pub struct Voice(pub String);

#[derive(Debug, Clone, serde::Serialize)]
#[serde(transparent)]
pub struct ResponseFormat(pub String);

impl Model {
    pub fn tts_1() -> Self {
        Self(String::from("tts-1"))
    }
    pub fn tts_1_hd() -> Self {
        Self(String::from("tts-1-hd"))
    }
}
impl Voice {
    pub fn alloy() -> Self {
        Self(String::from("alloy"))
    }
    pub fn echo() -> Self {
        Self(String::from("echo"))
    }
    pub fn fable() -> Self {
        Self(String::from("fable"))
    }
    pub fn onyx() -> Self {
        Self(String::from("onyx"))
    }
    pub fn nova() -> Self {
        Self(String::from("nova"))
    }
    pub fn shimmer() -> Self {
        Self(String::from("shimmer"))
    }
}
impl ResponseFormat {
    pub fn mp3() -> Self {
        Self(String::from("mp3"))
    }
    pub fn opus() -> Self {
        Self(String::from("opus"))
    }
    pub fn aac() -> Self {
        Self(String::from("aac"))
    }
    pub fn flac() -> Self {
        Self(String::from("flac"))
    }
    pub fn wav() -> Self {
        Self(String::from("wav"))
    }
    pub fn pcm() -> Self {
        Self(String::from("pcm"))
    }
}

impl From<String> for Model {
    fn from(value: String) -> Self { Self(value) }
}
impl From<String> for Voice {
    fn from(value: String) -> Self { Self(value) }
}
impl From<String> for ResponseFormat {
    fn from(value: String) -> Self { Self(value) }
}

impl From<&str> for Model {
    fn from(value: &str) -> Self { Self(value.to_owned()) }
}
impl From<&str> for Voice {
    fn from(value: &str) -> Self { Self(value.to_owned()) }
}
impl From<&str> for ResponseFormat {
    fn from(value: &str) -> Self { Self(value.to_owned()) }
}