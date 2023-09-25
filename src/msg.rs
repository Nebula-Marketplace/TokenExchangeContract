use cosmwasm_std::Uint128;
use std::vec::Vec;

pub struct Listing {
    amount: u64,
    price: Uint128, // price per token
    seller: String
}

pub struct InstantiateMsg {
    denom: String,
    fee: Uint128,
    symbol: String,
    decimals: u8,
    name: String,
    listed: Vec<Listing>
}