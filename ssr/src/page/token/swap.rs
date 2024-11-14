#[cfg(feature = "backend-admin")]
mod swap_prices {
    use candid::{encode_args, CandidType, Decode, Principal};
    use ic_agent::Agent;
    use leptos::{server, ServerFnError};
    use serde::Deserialize;
    use yral_canisters_client::{sns_root::CanisterIdRecord, user_index::CanisterStatusResponse};

    use crate::{
        page::token,
        state::{
            admin_canisters::{admin_canisters, AdminCanisters},
            canisters::unauth_canisters,
        },
    };

    #[derive(CandidType, Deserialize, PartialEq, Debug)]
pub struct PriceData{
    id: Nat,
    #[serde(rename = "volumeUSD1d")]
    volume_usd_1d: f64,
    #[serde(rename = "volumeUSD7d")]
    volume_usd_7d: f64,
    #[serde(rename = "totalVolumeUSD")]
    total_volume_usd: f64,
    name: String,
    #[serde(rename = "volumeUSD")]
    volume_usd: f64,
    #[serde(rename = "feesUSD")]
    fees_usd: f64,
    #[serde(rename = "priceUSDChange")]
    price_usd_change: f64,
    address: String,
    #[serde(rename = "txCount")]
    tx_count: u64,
    #[serde(rename = "priceUSD")]
    price_usd: f64,
    standard: String,
    symbol: String
}

const XRC_FETCHABLE_TOKENS_LEDGER_IDS:[&str; 4] = ["xevnm-gaaaa-aaaar-qafnq-cai", "mxzaz-hqaaa-aaaar-qaada-cai", "ss2fx-dyaaa-aaaar-qacoq-cai", "ryjl3-tyaaa-aaaaa-aaaba-cai"]; // [ckusdc, ckbtc, cketh, icp]
#[derive(CandidType, Deserialize)]
pub enum AssetClass { Cryptocurrency, FiatCurrency }

#[derive(CandidType, Deserialize)]
pub struct Asset { pub class: AssetClass, pub symbol: String }

#[derive(CandidType, Deserialize)]
pub struct GetExchangeRateRequest {
  pub timestamp: Option<u64>,
  pub quote_asset: Asset,
  pub base_asset: Asset,
}

#[derive(CandidType, Deserialize)]
pub struct ExchangeRateMetadata {
  pub decimals: u32,
  pub forex_timestamp: Option<u64>,
  pub quote_asset_num_received_rates: u64,
  pub base_asset_num_received_rates: u64,
  pub base_asset_num_queried_sources: u64,
  pub standard_deviation: u64,
  pub quote_asset_num_queried_sources: u64,
}

#[derive(CandidType, Deserialize)]
pub struct ExchangeRate {
  pub metadata: ExchangeRateMetadata,
  pub rate: u64,
  pub timestamp: u64,
  pub quote_asset: Asset,
  pub base_asset: Asset,
}

#[derive(CandidType, Deserialize)]
pub enum ExchangeRateError {
  AnonymousPrincipalNotAllowed,
  CryptoQuoteAssetNotFound,
  FailedToAcceptCycles,
  ForexBaseAssetNotFound,
  CryptoBaseAssetNotFound,
  StablecoinRateTooFewRates,
  ForexAssetsNotFound,
  InconsistentRatesReceived,
  RateLimited,
  StablecoinRateZeroRate,
  Other{ code: u32, description: String },
  ForexInvalidTimestamp,
  NotEnoughCycles,
  ForexQuoteAssetNotFound,
  StablecoinRateNotFound,
  Pending,
}

#[derive(CandidType, Deserialize)]
pub enum GetExchangeRateResult { Ok(ExchangeRate), Err(ExchangeRateError) }

#[derive(CandidType, Deserialize)]
pub struct PairInfoExt {
  pub id: String,
  #[serde(rename = "price0CumulativeLast")]
  pub price0_cumulative_last: candid::Nat,

  pub creator: Principal,
  pub reserve0: candid::Nat,
  pub reserve1: candid::Nat,
  pub lptoken: String,

  #[serde(rename = "totalSupply")]
  pub total_supply: candid::Nat,
  pub token0: String,
  pub token1: String,

  #[serde(rename = "price1CumulativeLast")]
  pub price1_cumulative_last: candid::Nat,
  #[serde(rename = "kLast")]
  pub k_last: candid::Nat,

  #[serde(rename = "blockTimestampLast")]
  pub block_timestamp_last: candid::Int,
}



    struct SwapPriceFetcher(Agent);

    impl SwapPriceFetcher {
        const XRC_FETCHABLE_TOKENS_LEDGER_IDS:[&str; 4] = ["xevnm-gaaaa-aaaar-qafnq-cai", "mxzaz-hqaaa-aaaar-qaada-cai", "ss2fx-dyaaa-aaaar-qacoq-cai", "ryjl3-tyaaa-aaaaa-aaaba-cai"]; // [ckusdc, ckbtc, cketh, icp]

        pub async fn get_token_price_from_token_creator(&self, token_ledger: Principal) -> Result<f64, ServerFnError> {
            let SwapPriceFetcher(agent) = self;

            let res = Decode!(
                &agent
                    .query(&Principal::management_canister(), "canister_status")
                    .with_arg(encode_args((CanisterIdRecord {
                        canister_id: *token_ledger,
                    },))?)
                    .call()
                    .await?,
                CanisterStatusResponse
            )?;

            let cans_wire = unauth_canisters();
            for controller in res
                .settings
                .controllers
                .into_iter()
                .filter(|pred| !pred.to_text().ends_with("-cai"))
            {
                let Ok(Some(user_canister)) = cans_wire
                    .get_individual_canister_by_user_principal(controller)
                    .await
                else {
                    continue;
                };

                let individual_user_cans = cans_wire.individual_user(user_canister).await;

                if let Some(cdao) = individual_user_cans
                    .deployed_cdao_canisters()
                    .await?
                    .into_iter()
                    .find(|cdao_cans| cdao_cans.ledger == token_ledger)
                {
                    return Ok(cdao.last_swapped_price);
                }
            }

            Err(ServerFnError::new("Token Price Not Found in Yral Network"))
        }

        pub async fn get_token_price_from_xrc(&self, token_ledger: Principal) -> Result<f64, ServerFnError>{
            let SwapPriceFetcher(agent) = self;

            if Self::XRC_FETCHABLE_TOKENS_LEDGER_IDS.contains(&token_ledger.to_text().as_str()){
                let cans_wire = unauth_canisters();
                let sns_ledger = cans_wire.sns_ledger(token_ledger).await;
                let symbol = sns_ledger.icrc_1_symbol().await?.replace("ck", "");

                let exchange_rate = Decode!(&agent.query(&Principal::from_text("uf6dk-hyaaa-aaaaq-qaaaq-cai").unwrap(), "get_exchange_rate").with_arg(encode_args(
                    (GetExchangeRateRequest{
                        base_asset:Asset{
                            class: AssetClass::Cryptocurrency,
                            symbol
                        },
                        quote_asset: Asset{
                            class: AssetClass::FiatCurrency,
                            symbol: "USD".to_string()
                        },
                        timestamp: None
                    }, )
                )?).call().await?, GetExchangeRateResult)?;
                return match exchange_rate{
                    GetExchangeRateResult::Ok(exchange_rate) => Ok(exchange_rate.rate as f64),
                    _ => Err(ServerFnError::new("Token Couldn't be found on the XRC"))
                }
            }

            Err(ServerFnError::new("XRC Unsupported Token"))
        }
    }
    #[server]
    pub async fn get_token_price(token_ledger: Principal) -> Result<f64, ServerFnError> {
        let admin_agent = admin_canisters();

        todo!()
    }
}
