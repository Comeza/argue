use std::fmt;
use std::time::Instant;

pub mod limiter;
pub mod openai;

/// Preamble on how the system should react.
pub const SYSTEM_MESSAGE_PRE: &str = include_str!("msg/sys_pre.txt");

/// Describe matters of fact
pub const SYSTEM_MESSAGE_FACT: &str = include_str!("msg/sysmsg_fact.txt");

/// Describe relation of ideas
pub const SYSTEM_MESSAGE_IMPLICATION: &str = include_str!("msg/sysmsg_implication.txt");

pub const IMPLICATION_PRE: &str = include_str!("msg/implication_pre.txt");
pub const IMPLICATION_MID: &str = include_str!("msg/implication_mid.txt");

pub enum ProofKind {
    Fact,
    Implication,
}

pub enum AiError {
    NoChoices,
    WrongFormat,
    Ratelimited,
    RewestError(reqwest::Error),
}

impl From<reqwest::Error> for AiError {
    fn from(error: reqwest::Error) -> Self {
        Self::RewestError(error)
    }
}

impl fmt::Display for AiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use AiError as E;
        match self {
            E::NoChoices => write!(f, "The AI gave no results."),
            E::WrongFormat => write!(f, "The AI returned in the wrong format."),
            E::Ratelimited => write!(f, "The AI is reatelimited."),
            E::RewestError(e) => write!(f, "Request Error: {e}"),
        }
    }
}

pub trait AiApi {
    fn is_rate_limited(&self) -> bool;
    fn rate_limit_till(&self) -> Instant;

    async fn check_fact(&mut self, input: String) -> Result<bool, AiError>;
    async fn check_implication(&mut self, premises: Vec<String>, conclusion: Vec<String>) -> Result<bool, AiError>;

    fn interprete_response(input: &str) -> Result<bool, AiError> {
        match input {
            x if x.starts_with("[TRUE]") => Ok(true),
            x if x.starts_with("[FALSE]") => Ok(false),
            _ => Err(AiError::WrongFormat),
        }
    }
}
