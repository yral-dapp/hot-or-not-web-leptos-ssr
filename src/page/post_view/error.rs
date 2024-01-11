use ic_agent::AgentError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PostViewError {
    #[error("IC agent error {0}")]
    Agent(#[from] AgentError),
    #[error("Canister error {0}")]
    Canister(String),
    #[error("http fetch error {0}")]
    HttpFetch(#[from] reqwest::Error),
}
