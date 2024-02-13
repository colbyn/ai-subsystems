//! The data model for ChatGPT (and ChatGPT compatible) responses.
//!
//! Supports both streaming and regular (batch) responses.
use serde::{Deserialize, Serialize};
use super::common::{Integer, Number};

/// The data model for reqular ChatGPT (and ChatGPT compatible) responses.
pub mod batch {
    use super::{FunctionCall, Integer, LogProbability, ToolCall};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Response {
        pub id: String,
        pub choices: Vec<Choice>,
        pub created: Integer,
        pub model: String,
        pub system_fingerprint: Option<String>,
        pub object: String,
        pub usage: Usage,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Choice {
        pub finish_reason: String,
        pub index: Integer,
        pub message: Message,
        pub logprobs: Option<LogProbability>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Message {
        pub content: Option<String>,
        pub tool_calls: Option<Vec<ToolCall>>,
        pub role: String,
        pub function_call: Option<FunctionCall>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Usage {
        pub completion_tokens: Integer,
        pub prompt_tokens: Integer,
        pub total_tokens: Integer,
    }
}

/// The data model for streaming ChatGPT (and ChatGPT compatible) responses.
pub mod streaming {
    use super::{FunctionCall, Integer, LogProbability, ToolCall};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ResponseChunk {
        pub id: String,
        pub choices: Vec<Choice>,
        pub created: Integer,
        pub model: String,
        pub system_fingerprint: Option<String>,
        pub object: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Choice {
        pub delta: MessageDelta,
        pub logprobs: Option<LogProbability>,
        pub finish_reason: Option<String>,
        pub index: Integer,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct MessageDelta {
        pub content: Option<String>,
        pub function_call: Option<FunctionCall>,
        pub tool_calls: Option<Vec<ToolCall>>,
        pub role: Option<String>,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub index: Integer,
    pub id: String,
    pub r#type: String,
    pub function: FunctionCall,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogProbability {
    pub content: Option<Vec<MessageLogProbability>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageLogProbability {
    pub token: String,
    pub logprob: Number,
    pub bytes: Vec<Integer>,
    pub top_logprobs: Vec<TopLogProbability>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopLogProbability {
    pub token: String,
    pub logprob: Number,
    pub bytes: Vec<Integer>,
}
