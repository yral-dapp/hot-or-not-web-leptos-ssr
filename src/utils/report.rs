use std::env;

use crate::consts::OFF_CHAIN_AGENT_GRPC_URL;
use crate::utils::off_chain;
use leptos::{server, ServerFnError};

pub enum ReportOption {
    Nudity,
    Violence,
    Offensive,
    Spam,
    Other,
}

impl ReportOption {
    pub fn as_str(&self) -> &'static str {
        match self {
            ReportOption::Nudity => "Nudity/Porn",
            ReportOption::Violence => "Violence/Gore",
            ReportOption::Offensive => "Offensive",
            ReportOption::Spam => "Spam/Ad",
            ReportOption::Other => "Others",
        }
    }
}

#[server]
pub async fn send_report_offchain(
    reporter_id: String,
    publisher_id: String,
    publisher_canister_id: String,
    post_id: String,
    video_id: String,
    reason: String,
    video_url: String,
) -> Result<(), ServerFnError> {
    use tonic::metadata::MetadataValue;
    use tonic::transport::Channel;
    use tonic::Request;

    let off_chain_agent_url = OFF_CHAIN_AGENT_GRPC_URL.as_ref();
    let channel = Channel::from_static(off_chain_agent_url).connect().await?;

    let mut off_chain_agent_grpc_auth_token = env::var("GRPC_AUTH_TOKEN").expect("GRPC_AUTH_TOKEN");
    // removing whitespaces and new lines for proper parsing
    off_chain_agent_grpc_auth_token.retain(|c| !c.is_whitespace());

    let token: MetadataValue<_> = format!("Bearer {}", off_chain_agent_grpc_auth_token).parse()?;

    let mut client = off_chain::off_chain_client::OffChainClient::with_interceptor(
        channel,
        move |mut req: Request<()>| {
            req.metadata_mut().insert("authorization", token.clone());
            Ok(req)
        },
    );

    let request = tonic::Request::new(off_chain::ReportPostRequest {
        reporter_id,
        publisher_id,
        publisher_canister_id,
        post_id,
        video_id,
        reason,
        video_url,
    });

    client.report_post(request).await?;

    Ok(())
}
