use http::Method;
use serde::{de, Deserialize, Deserializer, Serialize};
use serde_json::Value;

use super::{CfReqAuth, CfReqMeta};

#[derive(Serialize)]
pub struct VideoDetails {
    #[serde(skip)]
    identifier: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoStatus {
    pub error_reason_code: Option<String>,
    pub error_reason_text: Option<String>,
    pub pct_complete: Option<String>,
    pub state: String,
}

#[derive(Serialize, Deserialize)]
pub struct VideoDetailsRes {
    pub status: VideoStatus,
}

impl VideoDetails {
    pub fn new(identifier: impl Into<String>) -> Self {
        Self {
            identifier: identifier.into(),
        }
    }
}

impl CfReqMeta for VideoDetails {
    const METHOD: Method = Method::GET;
    type JsonResponse = VideoDetailsRes;
}

impl CfReqAuth for VideoDetails {
    type Url = String;

    fn path(&self, account_id: &str) -> String {
        format!("accounts/{account_id}/stream/{}", self.identifier)
    }
}
