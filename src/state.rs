use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Uint128;
use crate::msg::Listing;
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct State {
    pub denom: String,
    pub fee: Uint128,
    pub symbol: String,
    pub decimals: u8,
    pub name: String,
    pub listed: Vec<Listing>
}

pub const STATE: Item<State> = Item::new("state");