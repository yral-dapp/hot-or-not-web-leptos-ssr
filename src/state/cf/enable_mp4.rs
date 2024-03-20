use http::Method;
use serde::{Deserialize, Serialize};

use super::{CfReqAuth, CfReqMeta};

#[derive(Serialize)]
pub struct EnableMp4 {
    #[serde(skip)]
    identifier: String,
}

#[derive(Serialize, Deserialize)]
pub struct EnableMp4Res {}

impl EnableMp4 {
    pub fn new(identifier: impl Into<String>) -> Self {
        Self {
            identifier: identifier.into(),
        }
    }
}

impl CfReqMeta for EnableMp4 {
    const METHOD: Method = Method::POST;
    type JsonResponse = EnableMp4Res;
}

impl CfReqAuth for EnableMp4 {
    type Url = String;

    fn path(&self, account_id: &str) -> String {
        format!("accounts/{account_id}/stream/{}/downloads", self.identifier)
    }
}
