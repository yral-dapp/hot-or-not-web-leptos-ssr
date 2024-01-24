use super::CfApiErr;
use leptos::ServerFnErrorErr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error("errors from cloudflare: {0:?}")]
    Cloudflare(Vec<CfApiErr>),
}

impl From<Error> for ServerFnErrorErr {
    fn from(e: Error) -> Self {
        ServerFnErrorErr::ServerError(match e {
            Error::Reqwest(e) => {
                log::warn!("reqwest error while calling cf: {e}");
                "Failed to send to cloudflare".into()
            }
            Error::Cloudflare(_) => {
                log::warn!("{e}");
                "Cloudflare returned an error".into()
            }
        })
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
