use std::{collections::HashMap, time::Duration};

use super::{CfReqAuth, CfReqMeta};
use http::Method;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct Watermark {
    uid: String,
}

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DirectUpload {
    creator: Option<String>,
    max_duration_seconds: Option<u64>,
    meta: HashMap<String, String>,
    watermark: Option<Watermark>,
}

#[derive(Serialize, Deserialize)]
pub struct DirectUploadRes {
    pub uid: String,
    #[serde(rename = "uploadURL")]
    pub upload_url: String,
}

impl DirectUpload {
    pub fn creator(mut self, creator: impl Into<String>) -> Self {
        self.creator = Some(creator.into());
        self
    }

    pub fn max_duration(mut self, max_duration: Duration) -> Self {
        self.max_duration_seconds = Some(max_duration.as_secs());
        self
    }

    pub fn add_meta(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.meta.insert(key.into(), value.into());
        self
    }

    pub fn watermark(mut self, uid: impl Into<String>) -> Self {
        self.watermark = Some(Watermark { uid: uid.into() });
        self
    }
}

impl CfReqMeta for DirectUpload {
    const METHOD: Method = Method::POST;
    type JsonResponse = DirectUploadRes;
}

impl CfReqAuth for DirectUpload {
    type Url = String;

    fn path(&self, account_id: &str) -> String {
        format!("accounts/{account_id}/stream/direct_upload")
    }
}
