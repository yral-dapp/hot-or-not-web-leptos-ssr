pub mod icpump;

use std::{
    cmp::Ordering,
    ops::{Add, AddAssign, Sub, SubAssign},
    str::FromStr,
};

use candid::{Nat, Principal};
use ic_agent::AgentError;
use leptos::ServerFnError;
use rust_decimal::{Decimal, RoundingStrategy};
use serde::{Deserialize, Serialize};

use yral_canisters_client::{
    sns_governance::{DissolveState, GetMetadataArg, ListNeurons},
    sns_ledger::Account as LedgerAccount,
    sns_root::ListSnsCanistersArg,
};

use crate::state::canisters::Canisters;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TokenBalance {
    pub e8s: Nat,
    decimals: u8,
}

impl TokenBalance {
    pub fn new(e8s: Nat, decimals: u8) -> Self {
        Self { e8s, decimals }
    }

    /// Token Balance but with 8 decimals (default for Cdao)
    pub fn new_cdao(e8s: Nat) -> Self {
        Self::new(e8s, 8u8)
    }

    /// Parse a numeric value
    /// multiplied by 8 decimals (1e8)
    pub fn parse_cdao(token_str: &str) -> Result<Self, rust_decimal::Error> {
        let tokens = (Decimal::from_str(token_str)? * Decimal::new(1e8 as i64, 0)).floor();
        let e8s = Nat::from_str(&tokens.to_string()).unwrap();
        Ok(Self::new_cdao(e8s))
    }

    // Human friendly token amount
    pub fn humanize(&self) -> String {
        (self.e8s.clone() / 10u64.pow(self.decimals as u32))
            .to_string()
            .replace("_", ",")
    }

    // Humanize the amount, but as a float
    pub fn humanize_float(&self) -> String {
        let tokens = Decimal::from_str(&self.e8s.0.to_str_radix(10)).unwrap()
            / Decimal::new(10i64.pow(self.decimals as u32), 0);
        tokens.to_string()
    }

    // Humanize the amount, but as a truncated float to specified decimal points (dp)
    pub fn humanize_float_truncate_to_dp(&self, dp: u32) -> String {
        let tokens = Decimal::from_str(&self.e8s.0.to_str_radix(10)).unwrap()
            / Decimal::new(10i64.pow(self.decimals as u32), 0);
        tokens
            .round_dp_with_strategy(dp, RoundingStrategy::ToZero)
            .to_string()
    }

    // Returns number of tokens(not e8s)
    pub fn to_tokens(&self) -> String {
        let tokens = self.e8s.clone() / Nat::from(10u64.pow(self.decimals as u32));
        tokens.0.to_str_radix(10)
    }
}

impl From<TokenBalance> for Nat {
    fn from(value: TokenBalance) -> Nat {
        value.e8s
    }
}

impl<T> Add<T> for TokenBalance
where
    Nat: Add<T, Output = Nat>,
{
    type Output = Self;

    fn add(self, other: T) -> Self {
        Self {
            e8s: self.e8s + other,
            decimals: self.decimals,
        }
    }
}

impl<T> AddAssign<T> for TokenBalance
where
    Nat: AddAssign<T>,
{
    fn add_assign(&mut self, rhs: T) {
        self.e8s += rhs;
    }
}

impl<T> PartialEq<T> for TokenBalance
where
    Nat: PartialEq<T>,
{
    fn eq(&self, other: &T) -> bool {
        self.e8s.eq(other)
    }
}

impl<T> PartialOrd<T> for TokenBalance
where
    Nat: PartialOrd<T>,
{
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        self.e8s.partial_cmp(other)
    }
}

impl<T> Sub<T> for TokenBalance
where
    Nat: Sub<T, Output = Nat>,
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self {
        Self {
            e8s: self.e8s - rhs,
            decimals: self.decimals,
        }
    }
}

impl<T> SubAssign<T> for TokenBalance
where
    Nat: SubAssign<T>,
{
    fn sub_assign(&mut self, rhs: T) {
        self.e8s -= rhs;
    }
}

impl Sub for TokenBalance {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            e8s: self.e8s - rhs.e8s,
            decimals: self.decimals,
        }
    }
}

impl SubAssign<TokenBalance> for TokenBalance {
    fn sub_assign(&mut self, rhs: TokenBalance) {
        self.e8s -= rhs.e8s;
    }
}

impl PartialEq for TokenBalance {
    fn eq(&self, other: &Self) -> bool {
        self.e8s.eq(&other.e8s)
    }
}

impl PartialOrd for TokenBalance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.e8s.partial_cmp(&other.e8s)
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TokenBalanceOrClaiming(Option<TokenBalance>);

impl TokenBalanceOrClaiming {
    pub fn new(balance: TokenBalance) -> Self {
        Self(Some(balance))
    }

    pub fn claiming() -> Self {
        Self(None)
    }

    pub fn is_claiming(&self) -> bool {
        self.0.is_none()
    }

    pub fn humanize(&self) -> String {
        self.0
            .as_ref()
            .map(|b| b.humanize())
            .unwrap_or_else(|| "Processing".to_string())
    }

    pub fn humanize_float(&self) -> String {
        self.map_balance_ref(|b| b.humanize_float())
            .unwrap_or_else(|| "Processing".to_string())
    }

    pub fn humanize_float_truncate_to_dp(&self, dp: u32) -> String {
        self.map_balance_ref(|b| b.humanize_float_truncate_to_dp(dp))
            .unwrap_or_else(|| "Processing".to_string())
    }

    pub fn map_balance<T>(self, f: impl FnOnce(TokenBalance) -> T) -> Option<T> {
        self.0.map(f)
    }

    pub fn map_balance_ref<T>(&self, f: impl FnOnce(&TokenBalance) -> T) -> Option<T> {
        self.0.as_ref().map(f)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeployedCdaoCanisters {
    pub root: Principal,
    pub swap: Principal,
    pub ledger: Principal,
    pub index: Principal,
    pub governance: Principal,
}

impl From<yral_canisters_client::individual_user_template::DeployedCdaoCanisters>
    for DeployedCdaoCanisters
{
    fn from(value: yral_canisters_client::individual_user_template::DeployedCdaoCanisters) -> Self {
        Self {
            root: value.root,
            swap: value.swap,
            ledger: value.ledger,
            index: value.index,
            governance: value.governance,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TokenMetadata {
    pub logo_b64: String,
    pub name: String,
    pub description: String,
    pub symbol: String,
    pub balance: Option<TokenBalanceOrClaiming>,
    pub fees: TokenBalance,
    pub root: Principal,
    pub ledger: Principal,
    pub index: Principal,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TokenCans {
    pub governance: Principal,
    pub ledger: Principal,
    pub root: Principal,
}

pub async fn token_metadata_by_root<const A: bool>(
    cans: &Canisters<A>,
    user_principal: Option<Principal>,
    token_root: Principal,
) -> Result<Option<TokenMetadata>, ServerFnError> {
    // let user_principal = cans
    let root = cans.sns_root(token_root).await;
    let sns_cans = root.list_sns_canisters(ListSnsCanistersArg {}).await?;
    let Some(governance) = sns_cans.governance else {
        return Ok(None);
    };
    let Some(ledger) = sns_cans.ledger else {
        return Ok(None);
    };
    let Some(index) = sns_cans.index else {
        return Ok(None);
    };
    let metadata =
        get_token_metadata(cans, user_principal, token_root, governance, ledger, index).await?;

    Ok(Some(metadata))
}

pub async fn get_token_metadata<const A: bool>(
    cans: &Canisters<A>,
    user_principal: Option<Principal>,
    root: Principal,
    governance: Principal,
    ledger: Principal,
    index: Principal,
) -> Result<TokenMetadata, AgentError> {
    let governance_can = cans.sns_governance(governance).await;
    let metadata = governance_can.get_metadata(GetMetadataArg {}).await?;

    let ledger_can = cans.sns_ledger(ledger).await;
    let symbol = ledger_can.icrc_1_symbol().await?;

    let fees = ledger_can.icrc_1_fee().await?;

    let mut token_metadata = TokenMetadata {
        logo_b64: metadata.logo.unwrap_or_default(),
        name: metadata.name.unwrap_or_default(),
        description: metadata.description.unwrap_or_default(),
        symbol,
        fees: TokenBalance::new_cdao(fees),
        balance: None,
        root,
        ledger,
        index,
    };

    if let Some(user_principal) = user_principal {
        let balance = get_token_balance(cans, user_principal, governance, ledger).await?;
        token_metadata.balance = Some(balance);
    }

    Ok(token_metadata)
}

/// Fetches the token balance for an SNS token
/// returns TokenBalanceOrClaiming::Claiming if the token creation is in progress
async fn get_token_balance<const A: bool>(
    cans: &Canisters<A>,
    user_principal: Principal,
    governance: Principal,
    ledger: Principal,
) -> Result<TokenBalanceOrClaiming, AgentError> {
    let ledger = cans.sns_ledger(ledger).await;
    let acc = LedgerAccount {
        owner: user_principal,
        subaccount: None,
    };
    // Balance > 0 -> Token is already claimed
    let balance_e8s = ledger.icrc_1_balance_of(acc).await?;
    let ready_balance = |e8s| Ok(TokenBalanceOrClaiming::new(TokenBalance::new_cdao(e8s)));
    if balance_e8s > 0u8 {
        return ready_balance(balance_e8s);
    }

    // if balance is 0 we may not have completed claiming
    let governance = cans.sns_governance(governance).await;
    let neurons = governance
        .list_neurons(ListNeurons {
            of_principal: Some(user_principal),
            limit: 10,
            start_page_at: None,
        })
        .await?
        .neurons;

    if neurons.len() < 2 || neurons[1].cached_neuron_stake_e8s == 0 {
        return ready_balance(balance_e8s);
    }

    if matches!(
        neurons[1].dissolve_state.as_ref(),
        Some(DissolveState::DissolveDelaySeconds(0))
    ) {
        return Ok(TokenBalanceOrClaiming::claiming());
    }

    if neurons[0].cached_neuron_stake_e8s == 0 {
        return ready_balance(balance_e8s);
    }

    Ok(TokenBalanceOrClaiming::claiming())
}
