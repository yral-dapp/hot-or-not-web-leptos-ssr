use candid::Principal;
use http::StatusCode;
use ic_agent::identity::DelegatedIdentity;
use reqwest::Client;
use reqwest::Url;
use serde::Deserialize;
use serde::Serialize;
use yral_metadata_types::ApiResult;
use std::error::Error;

use crate::auth::DelegatedIdentityWire;

#[derive(Deserialize)]
pub struct AllowPrincpalRes{
    allowed: bool
}

#[derive(Serialize)]
pub struct UploadContentTestPayload {
    payload: DelegatedIdentityWire
}

#[derive(Serialize, Deserialize)]
pub struct UploadContentPayload {
    url: String,
    payload: DelegatedIdentityWire,
}

#[derive(Clone, Debug)]
pub struct ContentSeedClient {
    client: Client,
    base_url: Url,
}

impl ContentSeedClient {
    pub fn with_base_url(url: Url) -> Self {
        ContentSeedClient {
            client: Default::default(),
            base_url: url,
        }
    }

    pub async fn check_if_authorized(&self, principal: Principal) -> Result<bool, Box<dyn Error>> {
        let api_url = self.base_url.join("allowed/").expect("url error").join(&principal.to_string()).expect("url error");
        let res = self.client.get(api_url).send().await?;
        let res_json : AllowPrincpalRes = res.json().await?;
        Ok(res_json.allowed)
    }

    pub async fn upload_content(&self, url: String, identity: DelegatedIdentityWire) -> Result<(), Box<dyn Error>> {
        let api_url = self.base_url.join("test-payload/").expect("url join error");
        let req_body = UploadContentTestPayload {
            payload: identity
        };
        let res = self.client.post(api_url).json(&req_body).send().await?;
        match res.status() {
            StatusCode::ACCEPTED => Ok(()),
            _ => Err("Something went wrong".into())
        }
    }
}
