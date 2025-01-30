use std::env;

use leptos::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
pub mod nsfw_detector {
    tonic::include_proto!("nsfw_detector");
}

#[cfg(feature = "ssr")]
#[derive(Debug, Clone)]
pub struct ICPumpNSFWGrpcChannel {
    pub channel: tonic::transport::Channel,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct NSFWInfo {
    pub is_nsfw: bool,
    pub nsfw_ec: String,
    pub nsfw_gore: String,
    pub csam_detected: bool,
}

#[cfg(feature = "local-bin")]
#[server]
pub async fn get_nsfw_info(_base64_image: String) -> Result<NSFWInfo, ServerFnError> {
    Ok(Default::default())
}

#[cfg(not(feature = "local-bin"))]
#[server]
pub async fn get_nsfw_info(base64_image: String) -> Result<NSFWInfo, ServerFnError> {
    use tonic::metadata::MetadataValue;
    use tonic::Request;

    let channel: ICPumpNSFWGrpcChannel = expect_context();
    let nsfw_grpc_auth_token = env::var("NSFW_GRPC_TOKEN").expect("NSFW_GRPC_TOKEN");
    let token: MetadataValue<_> = format!("Bearer {}", nsfw_grpc_auth_token).parse()?;
    let mut client = nsfw_detector::nsfw_detector_client::NsfwDetectorClient::with_interceptor(
        channel.channel,
        move |mut req: Request<()>| {
            req.metadata_mut().insert("authorization", token.clone());
            Ok(req)
        },
    );

    let base64_image_without_prefix = base64_image.replace("data:image/png;base64,", "");

    let request = nsfw_detector::NsfwDetectorRequestImg {
        image: base64_image_without_prefix,
    };
    let resp: tonic::Response<nsfw_detector::NsfwDetectorResponse> =
        client.detect_nsfw_img(request).await?;

    let res = resp.into_inner();

    let nsfw_info: NSFWInfo = res.into();

    Ok(nsfw_info)
}

#[cfg(feature = "ssr")]
impl From<nsfw_detector::NsfwDetectorResponse> for NSFWInfo {
    fn from(item: nsfw_detector::NsfwDetectorResponse) -> Self {
        let is_nsfw = item.csam_detected
            || matches!(
                item.nsfw_gore.as_str(),
                "POSSIBLE" | "LIKELY" | "VERY_LIKELY"
            )
            || matches!(item.nsfw_ec.as_str(), "nudity" | "provocative" | "explicit");

        Self {
            is_nsfw,
            nsfw_ec: item.nsfw_ec,
            nsfw_gore: item.nsfw_gore,
            csam_detected: item.csam_detected,
        }
    }
}
