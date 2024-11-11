
use std::env;

use leptos::*;
use serde::{Serialize, Deserialize};

#[cfg(feature = "ssr")]
pub mod nsfw {
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
    pub nsfw_gore:  String,
    pub csam_detected: bool,
}

#[server]
pub async fn get_nsfw_info(base64_image: String) -> Result<NSFWInfo, ServerFnError> {
    use tonic::Request;
    use tonic::metadata::MetadataValue;

    let channel: ICPumpNSFWGrpcChannel = expect_context();
    let nsfw_grpc_auth_token = env::var("NSFW_GRPC_TOKEN").expect("NSFW_GRPC_TOKEN");
    let token: MetadataValue<_> = format!("Bearer {}", nsfw_grpc_auth_token).parse()?;
    let mut client = nsfw::nsfw_detector_client::NsfwDetectorClient::with_interceptor(
        channel.channel,
        move |mut req: Request<()>| {
            req.metadata_mut().insert("authorization", token.clone());
            Ok(req)
        },
    );

    let request = nsfw::NsfwDetectorRequestImg { image: base64_image };
    let resp: tonic::Response<nsfw::NsfwDetectorResponse> = client.detect_nsfw_img(request).await?;

    let res = resp.into_inner();

    let nsfw_info: NSFWInfo = res.into();

    Ok(nsfw_info)
}

#[cfg(feature = "ssr")]
impl From<nsfw::NsfwDetectorResponse> for NSFWInfo {
    fn from(item: nsfw::NsfwDetectorResponse) -> Self {
        let is_nsfw = item.csam_detected
            || matches!(item.nsfw_gore.as_str(), "POSSIBLE" | "LIKELY" | "VERY_LIKELY")
            || matches!(item.nsfw_ec.as_str(), "nudity" | "provocative" | "explicit");

        Self {
            is_nsfw,
            nsfw_ec: item.nsfw_ec,
            nsfw_gore: item.nsfw_gore,
            csam_detected: item.csam_detected,
        }
    }
}


