#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use cw2::set_contract_version;


use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};
use crate::error::ContractError;
use crate::execute::{buy, list, delist};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, "Vanguards Token Exchange", "v1")?;

    let state = State {
        denom: msg.denom,
        fee: msg.fee,
        symbol: msg.symbol,
        decimals: msg.decimals,
        name: msg.name,
        listed: vec![],
    };
    STATE.save(deps.storage, &state)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::List { price } => list(info.funds[0].amount.u128(), price, info.sender.into_string(), deps),
        ExecuteMsg::Buy { amount } => buy(amount, info.sender.into_string(), info.funds, deps),
        ExecuteMsg::Delist {} => delist(info.sender.into_string(), deps),
        // ExecuteMsg::LiquidatePool {} => liquidate_pool(info.sender.into_string(), deps),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: DepsMut, _env: Env, msg: QueryMsg) -> Result<Response, ContractError> {
    match msg {
        QueryMsg::GetOwned { address } => {
            let state = STATE.load(deps.storage)?;
            let mut owned: u128 = 0;
            for listing in state.listed {
                if listing.seller == address {
                    owned += listing.amount;
                }
            }
            Ok(Response::new().add_attribute("Owned", format!("{}", owned)))
        },
        QueryMsg::GetListed { } => {Ok(Response::new().add_attribute("Listed", format!("{:?}", STATE.load(deps.storage)?.listed)))},
        QueryMsg::GetState {} => {
            let state = STATE.load(deps.storage)?;
            Ok(Response::new().add_attribute("state", format!("{:?}", state)))
        }
    }
}