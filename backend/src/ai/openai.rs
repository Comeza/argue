use std::time::Instant;

use reqwest::{self};
use serde::{Deserialize, Serialize};

use super::limiter::TokenLimiter;
use super::{AiApi, AiError};

pub struct OpenAi {
    model: String,
    token: String,
    endpoint: String,
    limiter: TokenLimiter,
}

impl OpenAi {
    async fn request(&mut self, msgs: Vec<Message>) -> Result<String, AiError> {
        if self.is_rate_limited() {
            return Err(AiError::Ratelimited);
        }

        let request = msgs.into_iter().fold(OpenAiRequest::new(&self.model), |a, c| {
            a.append_message(c.role, c.content)
        });

        let response = reqwest::Client::new()
            .post(&self.endpoint)
            .bearer_auth(&self.token)
            .json(&request)
            .send()
            .await?
            .json::<OpenAiResponse>()
            .await?;

        if response.choices.is_empty() {
            return Err(AiError::NoChoices);
        }

        let content = response.choices.get(0).unwrap().message.content.to_owned();
        Ok(content)
    }
}

impl AiApi for OpenAi {
    fn is_rate_limited(&self) -> bool {
        self.limiter.is_limited()
    }

    fn rate_limit_till(&self) -> Instant {
        todo!()
    }

    async fn check_fact(&mut self, input: String) -> Result<bool, AiError> {
        let msgs = vec![Message::sys(super::SYSTEM_MESSAGE_PRE), Message::user(input)];
        let response = self.request(msgs).await?;
        Self::interprete_response(&response)
    }

    async fn check_implication(&mut self, premises: Vec<String>, conclusions: Vec<String>) -> Result<bool, AiError> {
        let mut msgs = vec![Message::sys(super::SYSTEM_MESSAGE_PRE)];

        premises
            .into_iter()
            .map(|msg| Message::user(format!("- {msg}")))
            .for_each(|m| msgs.push(m));

        msgs.push(Message::user(super::IMPLICATION_MID));

        conclusions
            .into_iter()
            .map(|msg| Message::user(format!("- {msg}")))
            .for_each(|m| msgs.push(m));

        let response = self.request(msgs).await?;
        Self::interprete_response(&response)
    }
}

impl OpenAiRequest {
    pub fn new(model: impl Into<String>) -> Self {
        Self {
            model: model.into(),
            response_format: None,
            messages: vec![],
        }
    }

    pub fn append_message(mut self, role: Role, msg: impl Into<String>) -> Self {
        self.messages.push(Message {
            role,
            content: msg.into(),
        });

        self
    }

    pub fn response_format(mut self, fmt: Option<ResponseFormat>) -> Self {
        self.response_format = fmt;
        self
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
struct OpenAiRequest {
    model: String,
    response_format: Option<ResponseFormat>,
    messages: Vec<Message>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
enum ResponseFormat {
    JsonObject,
    Text,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Role {
    System,
    User,
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    role: Role,
    content: String,
}

impl Message {
    pub fn new(role: Role, msg: impl Into<String>) -> Self {
        Self {
            role,
            content: msg.into(),
        }
    }
    pub fn user(msg: impl Into<String>) -> Self {
        Self::new(Role::User, msg)
    }

    pub fn sys(msg: impl Into<String>) -> Self {
        Self::new(Role::System, msg)
    }
}

#[derive(Deserialize, Debug)]
struct OpenAiResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
enum FinishReason {
    Stop,
    Length,
    ContentFilter,
    ToolCalls,
}

#[derive(Deserialize, Debug)]
struct Choice {
    message: Message,
}
