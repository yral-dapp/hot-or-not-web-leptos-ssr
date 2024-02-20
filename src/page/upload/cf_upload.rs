use candid::Principal;
#[cfg(all(feature = "cloudflare", feature = "ssr"))]
use cf_impl::server_func::*;
#[cfg(feature = "cloudflare")]
pub use cf_impl::upload_video_stream;
use leptos::*;
#[cfg(all(not(feature = "cloudflare"), feature = "ssr"))]
use mock_impl::server_func::*;
#[cfg(not(feature = "cloudflare"))]
pub use mock_impl::upload_video_stream;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UploadInfo {
    pub uid: String,
    pub upload_url: String,
}

#[server(GetUploadInfo)]
pub async fn get_upload_info(
    creator: Principal,
    hashtags: Vec<String>,
    description: String,
    file_name: String,
) -> Result<UploadInfo, ServerFnError> {
    // TODO(SECURITY): authenticate creator

    if description.len() < 10 {
        return Err(ServerFnError::Args(
            "Description must be at least 10 characters".into(),
        ));
    }
    if hashtags.len() > 8 {
        return Err(ServerFnError::Args("Too many hashtags".into()));
    }

    get_upload_info_impl(creator, hashtags, description, file_name).await
}

#[server(GetVideoStatus)]
pub async fn get_video_status(uid: String) -> Result<String, ServerFnError> {
    get_video_status_impl(uid).await
}

#[cfg(feature = "cloudflare")]
mod cf_impl {
    use super::UploadInfo;

    #[cfg(feature = "ssr")]
    pub mod server_func {
        use candid::Principal;
        use leptos::{expect_context, ServerFnError};

        use super::UploadInfo;
        use crate::{
            consts::CF_WATERMARK_UID,
            state::cf::{
                direct_upload::DirectUpload, enable_mp4::EnableMp4, video_details::VideoDetails,
                CfApi, CfReqAuth,
            },
        };
        use std::time::Duration;

        pub async fn get_upload_info_impl(
            creator: Principal,
            hashtags: Vec<String>,
            description: String,
            file_name: String,
        ) -> Result<UploadInfo, ServerFnError> {
            let cf_api: CfApi<true> = expect_context();
            let res = DirectUpload::default()
                .creator(creator.to_text())
                .add_meta("hashtags", hashtags.join(","))
                .add_meta("description", description)
                .add_meta("fileName", file_name)
                .add_meta("uploadType", "challenge")
                .watermark(CF_WATERMARK_UID)
                .max_duration(Duration::from_secs(60))
                .send(&cf_api)
                .await?;

            Ok(UploadInfo {
                uid: res.uid,
                upload_url: res.upload_url,
            })
        }

        pub async fn get_video_status_impl(uid: String) -> Result<String, ServerFnError> {
            let cf_api: CfApi<true> = expect_context();
            let res = VideoDetails::new(uid.clone()).send(&cf_api).await?;
            let state = res.status.state.as_str();
            if state == "ready" {
                EnableMp4::new(uid).send(&cf_api).await?;
            }

            Ok(res.status.state)
        }
    }

    pub async fn upload_video_stream(
        _upload_res: &UploadInfo,
        _file: &gloo::file::File,
    ) -> Result<(), gloo::net::Error> {
        #[cfg(feature = "hydrate")]
        {
            use gloo::net::http::Request;
            use leptos::web_sys::FormData;
            let form = FormData::new().unwrap();
            form.append_with_blob("file", _file.as_ref()).unwrap();
            let req = Request::post(&_upload_res.upload_url).body(form).unwrap();
            req.send().await?;
        }
        Ok(())
    }
}

#[cfg(not(feature = "cloudflare"))]
mod mock_impl {
    use super::UploadInfo;

    #[cfg(feature = "ssr")]
    pub mod server_func {
        use candid::Principal;
        use leptos::ServerFnError;
        use std::time::Duration;

        use super::UploadInfo;

        pub async fn get_upload_info_impl(
            _creator: Principal,
            _hashtags: Vec<String>,
            _description: String,
            _file_name: String,
        ) -> Result<UploadInfo, ServerFnError> {
            Ok(UploadInfo {
                uid: "mock".into(),
                upload_url: "http://mock.com".into(),
            })
        }

        pub async fn get_video_status_impl(_uid: String) -> Result<String, ServerFnError> {
            tokio::time::sleep(Duration::from_secs(2)).await;
            Ok("ready".into())
        }
    }

    pub async fn upload_video_stream(
        _upload_res: &UploadInfo,
        _file: &gloo::file::File,
    ) -> Result<(), gloo::net::Error> {
        use gloo::timers::future::TimeoutFuture;
        TimeoutFuture::new(1000).await;
        Ok(())
    }
}
