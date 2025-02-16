#[cfg(feature = "local-bin")]
pub mod containers;

use std::{
    env,
    fs::OpenOptions,
    io::{BufWriter, Write},
};

use axum_extra::extract::cookie::Key;
use leptos::LeptosOptions;
use leptos_router::RouteListing;

use crate::{
    auth::server_impl::store::KVStoreImpl,
    state::server::AppState,
    utils::token::{icpump::ICPumpSearchGrpcChannel, nsfw::ICPumpNSFWGrpcChannel},
};
use yral_canisters_common::Canisters;

#[cfg(feature = "cloudflare")]
fn init_cf() -> gob_cloudflare::CloudflareAuth {
    use gob_cloudflare::{CloudflareAuth, Credentials};
    let creds = Credentials {
        token: env::var("CF_TOKEN").expect("`CF_TOKEN` is required!"),
        account_id: env::var("CF_ACCOUNT_ID").expect("`CF_ACCOUNT_ID` is required!"),
    };
    CloudflareAuth::new(creds)
}

fn init_cookie_key() -> Key {
    let cookie_key_raw = {
        #[cfg(not(feature = "local-bin"))]
        {
            let cookie_key_str = env::var("COOKIE_KEY").expect("`COOKIE_KEY` is required!");
            hex::decode(cookie_key_str).expect("Invalid `COOKIE_KEY` (must be length 128 hex)")
        }
        #[cfg(feature = "local-bin")]
        {
            use rand_chacha::rand_core::{OsRng, RngCore};
            let mut cookie_key = [0u8; 64];
            OsRng.fill_bytes(&mut cookie_key);
            cookie_key.to_vec()
        }
    };
    Key::from(&cookie_key_raw)
}

#[cfg(feature = "oauth-ssr")]
fn init_google_oauth() -> crate::auth::core_clients::CoreClients {
    use crate::auth::core_clients::CoreClients;
    use crate::consts::google::GOOGLE_ISSUER_URL;
    use openidconnect::core::CoreProviderMetadata;
    use openidconnect::{
        core::CoreClient, reqwest::http_client, ClientId, ClientSecret, IssuerUrl, RedirectUrl,
    };

    let client_id = env::var("GOOGLE_CLIENT_ID").expect("`GOOGLE_CLIENT_ID` is required!");
    let client_secret =
        env::var("GOOGLE_CLIENT_SECRET").expect("`GOOGLE_CLIENT_SECRET` is required!");
    let redirect_uri = env::var("GOOGLE_REDIRECT_URL").expect("`GOOGLE_REDIRECT_URL` is required!");

    let google_oauth_metadata = CoreProviderMetadata::discover(
        &IssuerUrl::new(GOOGLE_ISSUER_URL.to_string()).unwrap(),
        http_client,
    )
    .unwrap();

    let google_oauth = CoreClient::from_provider_metadata(
        google_oauth_metadata.clone(),
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_uri).unwrap());

    let client_id =
        env::var("HOTORNOT_GOOGLE_CLIENT_ID").expect("`HOTORNOT_GOOGLE_CLIENT_ID` is required!");
    let client_secret = env::var("HOTORNOT_GOOGLE_CLIENT_SECRET")
        .expect("`HOTORNOT_GOOGLE_CLIENT_SECRET` is required!");
    let redirect_uri = env::var("HOTORNOT_GOOGLE_REDIRECT_URL")
        .expect("`HOTORNOT_GOOGLE_REDIRECT_URL` is required!");

    let hotornot_google_oauth = CoreClient::from_provider_metadata(
        google_oauth_metadata.clone(),
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_uri).unwrap());

    let client_id =
        env::var("ICPUMPFUN_GOOGLE_CLIENT_ID").expect("`ICPUMPFUN_GOOGLE_CLIENT_ID` is required!");
    let client_secret = env::var("ICPUMPFUN_GOOGLE_CLIENT_SECRET")
        .expect("`ICPUMPFUN_GOOGLE_CLIENT_SECRET` is required!");
    let redirect_uri = env::var("ICPUMPFUN_GOOGLE_REDIRECT_URL")
        .expect("`ICPUMPFUN_GOOGLE_REDIRECT_URL` is required!");

    let icpump_google_oauth = CoreClient::from_provider_metadata(
        google_oauth_metadata.clone(),
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_uri).unwrap());

    let client_id =
        env::var("PUMPDUMP_GOOGLE_CLIENT_ID").expect("`PUMPDUMP_GOOGLE_CLIENT_ID` is required!");
    let client_secret = env::var("PUMPDUMP_GOOGLE_CLIENT_SECRET")
        .expect("`PUMPDUMP_GOOGLE_CLIENT_SECRET` is required!");
    let redirect_uri = env::var("PUMPDUMP_GOOGLE_REDIRECT_URL")
        .expect("`PUMPDUMP_GOOGLE_REDIRECT_URL` is required!");

    let pumpdump_google_oauth = CoreClient::from_provider_metadata(
        google_oauth_metadata.clone(),
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_uri).unwrap());

    CoreClients {
        google_oauth,
        hotornot_google_oauth,
        icpump_google_oauth,
        pumpdump_google_oauth,
    }
}

#[cfg(feature = "firestore")]
async fn init_firestoredb() -> firestore::FirestoreDb {
    use firestore::{FirestoreDb, FirestoreDbOptions};

    // firestore-rs needs the service account key to be in a file
    let sa_key_file = env::var("HON_GOOGLE_SERVICE_ACCOUNT").expect("HON_GOOGLE_SERVICE_ACCOUNT");
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("hon_google_service_account.json")
        .expect("create file");

    let mut f = BufWriter::new(file);
    f.write_all(sa_key_file.as_bytes()).expect("write file");
    f.flush().expect("flush file");

    env::set_var(
        "GOOGLE_APPLICATION_CREDENTIALS",
        "hon_google_service_account.json",
    );

    let options = FirestoreDbOptions::new("hot-or-not-feed-intelligence".to_string())
        .with_database_id("ic-pump-fun".to_string());

    FirestoreDb::with_options(options)
        .await
        .expect("failed to create db")
}

#[cfg(feature = "ga4")]
async fn init_grpc_offchain_channel() -> tonic::transport::Channel {
    use crate::consts::OFF_CHAIN_AGENT_GRPC_URL;
    use tonic::transport::{Channel, ClientTlsConfig};

    let tls_config = ClientTlsConfig::new().with_webpki_roots();
    let off_chain_agent_url = OFF_CHAIN_AGENT_GRPC_URL.as_ref();
    Channel::from_static(off_chain_agent_url)
        .tls_config(tls_config)
        .expect("Couldn't update TLS config for off-chain agent")
        .connect()
        .await
        .expect("Couldn't connect to off-chain agent")
}

async fn init_grpc_icpump_search_channel() -> ICPumpSearchGrpcChannel {
    use crate::consts::ICPUMP_SEARCH_GRPC_URL;
    use tonic::transport::{Channel, ClientTlsConfig};

    let tls_config = ClientTlsConfig::new().with_webpki_roots();
    let off_chain_agent_url = ICPUMP_SEARCH_GRPC_URL;
    let channel = Channel::from_static(off_chain_agent_url)
        .tls_config(tls_config)
        .expect("Couldn't update TLS config for off-chain agent")
        .connect()
        .await
        .expect("Couldn't connect to off-chain agent");

    ICPumpSearchGrpcChannel { channel }
}

async fn init_grpc_nsfw_channel() -> ICPumpNSFWGrpcChannel {
    use crate::consts::NSFW_SERVER_URL;
    use tonic::transport::{Channel, ClientTlsConfig};

    let tls_config = ClientTlsConfig::new().with_webpki_roots();
    let channel = Channel::from_static(NSFW_SERVER_URL)
        .tls_config(tls_config)
        .expect("Couldn't update TLS config for nsfw agent")
        .connect()
        .await
        .expect("Couldn't connect to nsfw agent");

    ICPumpNSFWGrpcChannel { channel }
}

#[cfg(feature = "backend-admin")]
fn init_admin_canisters() -> crate::state::admin_canisters::AdminCanisters {
    use crate::state::admin_canisters::AdminCanisters;

    #[cfg(feature = "local-bin")]
    {
        use ic_agent::identity::Secp256k1Identity;
        use k256::SecretKey;
        use yral_testcontainers::backend::ADMIN_SECP_BYTES;

        let sk = SecretKey::from_bytes(&ADMIN_SECP_BYTES.into()).unwrap();
        let identity = Secp256k1Identity::from_private_key(sk);
        AdminCanisters::new(identity)
    }

    #[cfg(not(feature = "local-bin"))]
    {
        use ic_agent::identity::BasicIdentity;

        let admin_id_pem =
            env::var("BACKEND_ADMIN_IDENTITY").expect("`BACKEND_ADMIN_IDENTITY` is required!");
        let admin_id_pem_by = admin_id_pem.as_bytes();
        let admin_id =
            BasicIdentity::from_pem(admin_id_pem_by).expect("Invalid `BACKEND_ADMIN_IDENTITY`");
        AdminCanisters::new(admin_id)
    }
}

#[cfg(feature = "qstash")]
fn init_qstash_client() -> crate::utils::qstash::QStashClient {
    use crate::utils::qstash::QStashClient;

    let auth_token = env::var("QSTASH_TOKEN").expect("`QSTASH_TOKEN` is required!");

    QStashClient::new(&auth_token)
}

pub struct AppStateRes {
    pub app_state: AppState,
    #[cfg(feature = "local-bin")]
    pub containers: containers::TestContainers,
}

pub struct AppStateBuilder {
    leptos_options: LeptosOptions,
    routes: Vec<RouteListing>,
    #[cfg(feature = "local-bin")]
    containers: containers::TestContainers,
}

impl AppStateBuilder {
    pub fn new(leptos_options: LeptosOptions, routes: Vec<RouteListing>) -> Self {
        Self {
            leptos_options,
            routes,
            #[cfg(feature = "local-bin")]
            containers: containers::TestContainers::default(),
        }
    }

    async fn init_kv(&mut self) -> KVStoreImpl {
        #[cfg(feature = "redis-kv")]
        {
            use crate::auth::server_impl::store::redis_kv::RedisKV;
            let redis_url: String;
            #[cfg(feature = "local-bin")]
            {
                self.containers.start_redis().await;
                redis_url = "redis://127.0.0.1:6379".to_string();
            }
            #[cfg(not(feature = "local-bin"))]
            {
                redis_url = env::var("REDIS_URL").expect("`REDIS_URL` is required!");
            }
            KVStoreImpl::Redis(RedisKV::new(&redis_url).await.unwrap())
        }

        #[cfg(not(feature = "redis-kv"))]
        {
            use crate::auth::server_impl::store::redb_kv::ReDBKV;
            KVStoreImpl::ReDB(ReDBKV::new().expect("Failed to initialize ReDB"))
        }
    }

    pub async fn build(mut self) -> AppStateRes {
        let kv = self.init_kv().await;
        #[cfg(feature = "local-bin")]
        {
            self.containers.start_backend().await;
            self.containers.start_metadata().await;
        }

        let app_state = AppState {
            leptos_options: self.leptos_options,
            canisters: Canisters::default(),
            routes: self.routes,
            #[cfg(feature = "backend-admin")]
            admin_canisters: init_admin_canisters(),
            #[cfg(feature = "cloudflare")]
            cloudflare: init_cf(),
            kv,
            cookie_key: init_cookie_key(),
            #[cfg(feature = "oauth-ssr")]
            google_oauth_clients: init_google_oauth(),
            #[cfg(feature = "ga4")]
            grpc_offchain_channel: init_grpc_offchain_channel().await,
            #[cfg(feature = "firestore")]
            firestore_db: init_firestoredb().await,
            #[cfg(feature = "qstash")]
            qstash: init_qstash_client(),
            grpc_icpump_search_channel: init_grpc_icpump_search_channel().await,
            grpc_nsfw_channel: init_grpc_nsfw_channel().await,
        };

        AppStateRes {
            app_state,
            #[cfg(feature = "local-bin")]
            containers: self.containers,
        }
    }
}
