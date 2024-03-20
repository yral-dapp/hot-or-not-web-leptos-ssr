use std::{env, sync::Arc};

use futures::Future;
use http::Method;
use reqwest::{IntoUrl, RequestBuilder};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::consts::CF_BASE_URL;

pub mod direct_upload;
pub mod enable_mp4;
mod error;
pub mod video_details;
pub use error::*;

#[derive(Debug)]
pub struct CfCredentials {
    token: String,
    account_id: String,
}

impl CfCredentials {
    pub fn from_env(token_env: &str, account_id_env: &str) -> Option<Self> {
        Some(Self {
            token: env::var(token_env).ok()?,
            account_id: env::var(account_id_env).ok()?,
        })
    }
}

#[derive(Clone, Debug)]
pub struct CfApi<const AUTHENTICATED: bool> {
    client: reqwest::Client,
    credentials: Option<Arc<CfCredentials>>,
}

impl CfApi<true> {
    pub fn new(creds: CfCredentials) -> Self {
        Self {
            client: Default::default(),
            credentials: Some(Arc::new(creds)),
        }
    }

    async fn send_auth<Req: CfReqAuth>(&self, req: Req) -> Result<Req::JsonResponse> {
        let reqb = self.req_builder(
            Req::METHOD,
            CF_BASE_URL
                .join(
                    req.path(&self.credentials.as_ref().unwrap().account_id)
                        .as_ref(),
                )
                .unwrap(),
        );
        self.send_inner(req, reqb).await
    }
}

impl Default for CfApi<false> {
    fn default() -> Self {
        Self::new()
    }
}

impl CfApi<false> {
    pub fn new() -> Self {
        Self {
            client: Default::default(),
            credentials: None,
        }
    }
}

impl<const AUTH: bool> CfApi<AUTH> {
    fn req_builder(&self, method: Method, url: impl IntoUrl) -> RequestBuilder {
        let reqb = self.client.request(method, url);
        if let Some(creds) = self.credentials.as_ref() {
            reqb.bearer_auth(&creds.token)
        } else {
            reqb
        }
    }

    async fn send_inner<Req: CfReqMeta>(
        &self,
        req: Req,
        reqb: RequestBuilder,
    ) -> Result<Req::JsonResponse> {
        let reqb = if Req::METHOD == Method::GET {
            reqb.query(&req)
        } else {
            reqb.json(&req)
        };

        let resp = reqb.send().await?;
        let status = resp.status();
        if status.is_success() {
            let res: CfSuccessRes<Req::JsonResponse> = resp.json().await?;
            return Ok(res.result);
        }
        let err: CfErrRes = resp.json().await?;
        Err(Error::Cloudflare(err.errors))
    }

    async fn send<Req: CfReq>(&self, req: Req) -> Result<Req::JsonResponse> {
        let reqb = self.req_builder(Req::METHOD, CF_BASE_URL.join(Req::PATH).unwrap());
        self.send_inner(req, reqb).await
    }
}

pub trait CfReqMeta: Serialize + Sized + Send {
    const METHOD: Method;
    type JsonResponse: DeserializeOwned;
}

pub trait CfReq: CfReqMeta {
    const PATH: &'static str;

    fn send<const AUTH: bool>(
        self,
        api: &CfApi<AUTH>,
    ) -> impl Future<Output = Result<Self::JsonResponse>> {
        api.send(self)
    }
}

pub trait CfReqAuth: CfReqMeta {
    type Url: AsRef<str>;

    fn path(&self, account_id: &str) -> Self::Url;

    fn send(self, api: &CfApi<true>) -> impl Future<Output = Result<Self::JsonResponse>> {
        api.send_auth(self)
    }
}

#[derive(Deserialize, Debug)]
pub struct CfApiErr {
    pub code: u16,
    pub message: String,
}

#[derive(Deserialize)]
struct CfSuccessRes<T> {
    result: T,
}

#[derive(Deserialize)]
struct CfErrRes {
    errors: Vec<CfApiErr>,
}
