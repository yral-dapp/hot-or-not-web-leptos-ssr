#[cfg(feature = "backend-admin")]
mod swap_prices {
    use candid::{encode_args, CandidType, Decode, Nat, Principal};
    use ic_agent::Agent;
    use leptos::{server, ServerFnError};
    use serde::Deserialize;
    use tokio::join;
    use yral_canisters_client::{sns_root::CanisterIdRecord, user_index::CanisterStatusResponse};

    use crate::state::{admin_canisters::admin_canisters, canisters::unauth_canisters};
    use std::str::FromStr;
    #[derive(CandidType, Deserialize, PartialEq, Debug)]
    pub struct PriceData {
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
        symbol: String,
    }

    #[derive(CandidType, Deserialize)]
    pub enum AssetClass {
        Cryptocurrency,
        FiatCurrency,
    }

    #[derive(CandidType, Deserialize)]
    pub struct Asset {
        pub class: AssetClass,
        pub symbol: String,
    }

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

    #[derive(CandidType, Deserialize, Debug)]
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
        Other { code: u32, description: String },
        ForexInvalidTimestamp,
        NotEnoughCycles,
        ForexQuoteAssetNotFound,
        StablecoinRateNotFound,
        Pending,
    }

    #[derive(CandidType, Deserialize)]
    pub enum GetExchangeRateResult {
        Ok(ExchangeRate),
        Err(ExchangeRateError),
    }

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

    struct SwapPriceFetcher<'a>(&'a Agent);

    impl<'a> SwapPriceFetcher<'a> {
        const XRC_FETCHABLE_TOKENS_LEDGER_IDS: [&'static str; 4] = [
            "xevnm-gaaaa-aaaar-qafnq-cai",
            "mxzaz-hqaaa-aaaar-qaada-cai",
            "ss2fx-dyaaa-aaaar-qacoq-cai",
            "ryjl3-tyaaa-aaaaa-aaaba-cai",
        ]; // [ckusdc, ckbtc, cketh, icp]

        pub async fn get_token_price_from_token_creator(
            &self,
            token_ledger: Principal,
        ) -> Result<f64, ServerFnError> {
            let SwapPriceFetcher(agent) = self;

            let res = Decode!(
                &agent
                    .query(&Principal::management_canister(), "canister_status")
                    .with_arg(encode_args((CanisterIdRecord {
                        canister_id: token_ledger,
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

            Err(ServerFnError::new("No token creator found from yral network"))
        }

        pub async fn get_token_price_from_xrc(
            &self,
            token_ledger: Principal,
        ) -> Result<f64, ServerFnError> {
            let SwapPriceFetcher(agent) = self;

            if Self::XRC_FETCHABLE_TOKENS_LEDGER_IDS.contains(&token_ledger.to_text().as_str()) {
                let cans_wire = unauth_canisters();
                let sns_ledger = cans_wire.sns_ledger(token_ledger).await;
                let symbol = sns_ledger.icrc_1_symbol().await?.replace("ck", "");

                let exchange_rate = Decode!(
                    &agent
                        .query(
                            &Principal::from_text("uf6dk-hyaaa-aaaaq-qaaaq-cai")?,
                            "get_exchange_rate"
                        )
                        .with_arg(encode_args((GetExchangeRateRequest {
                            base_asset: Asset {
                                class: AssetClass::Cryptocurrency,
                                symbol
                            },
                            quote_asset: Asset {
                                class: AssetClass::FiatCurrency,
                                symbol: "USD".to_string()
                            },
                            timestamp: None
                        },))?)
                        .call()
                        .await?,
                    GetExchangeRateResult
                )?;
                return match exchange_rate {
                    GetExchangeRateResult::Ok(exchange_rate) => Ok(exchange_rate.rate as f64),
                    GetExchangeRateResult::Err(e) => Err(ServerFnError::new(format!("{:?}", e))),
                };
            }

            Err(ServerFnError::new(""))
        }

        pub async fn get_token_price_from_icpswap(
            &self,
            token_ledger: Principal,
        ) -> Result<f64, ServerFnError> {
            let SwapPriceFetcher(agent) = self;

            let price = Decode!(
                &agent
                    .query(
                        &Principal::from_text("moe7a-tiaaa-aaaag-qclfq-cai")?,
                        "getToken"
                    )
                    .with_arg(encode_args((token_ledger.to_text(),))?)
                    .call()
                    .await?,
                PriceData
            )?;

            Ok(price.price_usd)
        }

        pub async fn get_token_price_from_sonicswap(
            &self,
            token_ledger: Principal,
        ) -> Result<f64, ServerFnError> {
            let SwapPriceFetcher(agent) = self;
            let icp_ledger = Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai")?;

            let Some(token_icp_pair) = Decode!(
                &agent
                    .query(
                        &Principal::from_text("3xwpq-ziaaa-aaaah-qcn4a-cai")?,
                        "getPair",
                    )
                    .with_arg(encode_args((token_ledger, icp_ledger,))?)
                    .call()
                    .await?,
                Option<PairInfoExt>
            )? else{
                return Err(ServerFnError::new("Token not found"))
            };

            let icp_price = self.get_token_price_from_xrc(icp_ledger).await?;

            let reserve0_f64 = Self::nat_to_f64(token_icp_pair.reserve0)?;
            let reserve1_f64 = Self::nat_to_f64(token_icp_pair.reserve1)?;

            if reserve0_f64 == 0.0 {
                return Err(ServerFnError::new("Something went wrong, reserve0 is 0.0"));
            }

            let token_price_in_icp = reserve1_f64 / reserve0_f64;

            let token_price_in_usd = token_price_in_icp * icp_price;

            Ok(token_price_in_usd)
        }

        fn nat_to_f64(n: Nat) -> Result<f64, ServerFnError> {
            let n_str = n.to_string();
            f64::from_str(&n_str).map_err(|e| ServerFnError::new(format!("{:?}", e)))
        }
    }
    #[server]
    pub async fn fetch_and_update_token_price(
        token_ledger: Principal,
        requestee_user_canister: Principal,
    ) -> Result<(), ServerFnError>{
        let admin_agent = admin_canisters();

        let fetcher = SwapPriceFetcher(admin_agent.get_agent().await);

        let price = fetcher
            .get_token_price_from_token_creator(token_ledger)
            .await
            .or(fetcher.get_token_price_from_xrc(token_ledger).await)
            .or({
                let (icpswap_price, sonicswap_price) = join!(
                    fetcher.get_token_price_from_icpswap(token_ledger),
                    fetcher.get_token_price_from_sonicswap(token_ledger)
                );

                if icpswap_price.is_ok() && sonicswap_price.is_ok() {
                    Ok((icpswap_price.unwrap() + sonicswap_price.unwrap()) / 2.0)
                } else {
                    icpswap_price.or(sonicswap_price)
                }
            });

        admin_agent
            .individual_user_for(requestee_user_canister)
            .await
            .update_last_swapped_price(token_ledger, price.unwrap_or(0.0))
            .await;

        Ok(())
    }
}
