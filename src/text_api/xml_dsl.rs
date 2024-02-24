use std::{path::Path, str::FromStr};

use super::client::Error;
pub use liquid::object;

#[derive(Debug, Clone)]
pub struct PromptCollection {
    prompts: Vec<Prompt>,
}

#[derive(Debug, Clone)]
pub struct Prompt {
    pub name: Option<String>,
    pub request: super::request::RequestBuilder,
}

impl PromptCollection {
    pub fn open(file_path: impl AsRef<Path>, globals: &dyn liquid::ObjectView) -> Result<Self, Error> {
        let source = std::fs::read_to_string(file_path.as_ref())?;
        Self::parse(source, globals)
    }
    pub fn parse(contents: impl AsRef<str>, globals: &dyn liquid::ObjectView) -> Result<Self, Error> {
        // let contents = std::fs::read_to_string(file_path.as_ref());
        let source = contents.as_ref();
        let source = liquid::ParserBuilder::with_stdlib()
            .build()
            .unwrap()
            .parse(&source)
            .unwrap();
        let source = source.render(&globals).unwrap();
        let html = scraper::Html::parse_fragment(&source);
        let selector = scraper::Selector::parse("prompt").unwrap();
        let prompts = html
            .select(&selector)
            .filter_map(process_prompt_element)
            .collect::<Vec<_>>();
        Ok(PromptCollection { prompts })
    }
    pub fn get(&self, prompt_name: impl AsRef<str>) -> Option<Prompt> {
        let target = prompt_name.as_ref();
        for prompt in self.prompts.iter() {
            if let Some(name) = prompt.name.as_ref() {
                if name == &target {
                    return Some(prompt.clone());
                }
            }
        }
        None
    }
}

impl Prompt {
    pub fn open(file_path: impl AsRef<Path>, prompt_name: impl AsRef<str>, globals: &dyn liquid::ObjectView) -> Result<Self, Error> {
        let prompt_name = prompt_name.as_ref();
        let collection = PromptCollection::open(file_path, globals)?;
        let prompt = collection.get(prompt_name)
            .ok_or(Box::new(PromptNotFound(prompt_name.to_string())))?;
        Ok(prompt)
    }
    pub fn parse(contents: impl AsRef<str>, prompt_name: impl AsRef<str>, globals: &dyn liquid::ObjectView) -> Result<Self, Error> {
        let prompt_name = prompt_name.as_ref();
        let collection = PromptCollection::parse(contents, globals)?;
        let prompt = collection.get(prompt_name)
            .ok_or(Box::new(PromptNotFound(prompt_name.to_string())))?;
        Ok(prompt)
    }
    pub fn client_builder(self) -> super::client::ApiCallBuilder {
        super::client::ApiCallBuilder::default()
            .with_request_body(self.request)
    }
}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
#[derive(Debug, Clone)]
pub struct PromptNotFound(pub String);
impl std::fmt::Display for PromptNotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cannot find prompt: {:?}.", self.0)
    }
}
impl std::error::Error for PromptNotFound {}

//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
// TODO
//―――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――
fn process_prompt_element(element: scraper::ElementRef) -> Option<Prompt> {
    let name = element.attr("name")
        .map(str::to_string);
    let model = element.attr("model")
        .map(str::to_string);
    let stream = element.attr("stream")
        .and_then(|x| bool::from_str(&x).ok());
    let temperature = element.attr("temperature")
        .and_then(|x| f32::from_str(&x).ok());
    let n = element.attr("n")
        .and_then(|x| super::common::Integer::from_str(&x).ok());
    let max_tokens = element.attr("max-tokens")
        .and_then(|x| super::common::Integer::from_str(&x).ok());
    let top_p = element.attr("top-p")
        .and_then(|x| f32::from_str(&x).ok());
    let frequency_penalty = element.attr("frequency-penalty")
        .and_then(|x| f32::from_str(&x).ok());
    let presence_penalty = element.attr("presence-penalty")
        .and_then(|x| f32::from_str(&x).ok());
    let logprobs = element.attr("logprobs")
        .and_then(|x| bool::from_str(&x).ok());
    let top_logprobs = element.attr("top-logprobs")
        .and_then(|x| super::common::Integer::from_str(&x).ok());
    let response_format = element
        .attr("response-format")
        .and_then(|x| {
            match x.to_lowercase().as_str() {
                "json-object" => Some(super::request::ResponseFormat::JSON_OBJECT),
                "json_object" => Some(super::request::ResponseFormat::JSON_OBJECT),
                "text" => Some(super::request::ResponseFormat::TEXT),
                _ => None
            }
        });
    // let stop = element.attr("stop").map(str::to_string);
    // - * -
    let mut request = super::request::RequestBuilder::default();
    request.model = model;
    request.stream = stream;
    request.temperature = temperature;
    request.n = n;
    request.max_tokens = max_tokens;
    request.top_p = top_p;
    request.frequency_penalty = frequency_penalty;
    request.presence_penalty = presence_penalty;
    request.logprobs = logprobs;
    request.top_logprobs = top_logprobs;
    request.response_format = response_format;
    // - * -
    let message_selector = scraper::Selector::parse("message").unwrap();
    let messages = element
        .select(&message_selector)
        .map(|message_element| -> super::request::Message {
            let content = message_element.inner_html().trim().to_string();
            let content = unindent::unindent(&content);
            let role: &str = message_element.attr("role").unwrap_or("user");
            match role {
                "system" => {
                    return super::request::Message::system(content)
                }
                "assistant" => {
                    return super::request::Message::assistant(content)
                }
                "user" => {
                    return super::request::Message::user(content)
                }
                _ => return super::request::Message::user(content)
            }
        })
        .collect::<Vec<_>>();
    request.messages = messages;
    // - * -
    let prompt = Prompt { name, request };
    Some(prompt)
}



