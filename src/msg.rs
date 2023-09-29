use cosmwasm_std::Uint128;
use std::vec::Vec;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Listing {
    pub amount: u128,
    pub price: Uint128, // price per token
    pub seller: String
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct InstantiateMsg {
    pub denom: String,
    pub fee: Uint128,
    pub symbol: String,
    pub decimals: u8,
    pub name: String,
    pub listed: Vec<Listing>
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum ExecuteMsg {
    List {
        price: Uint128
    },
    Buy {
        amount: u128,
    },
    Delist {},
    // LiquidatePool {}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum QueryMsg {
    GetState {},
    GetOwned {
        address: String
    },
    GetListed {}
}