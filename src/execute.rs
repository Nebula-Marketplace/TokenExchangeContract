use cosmwasm_std::{to_binary, DepsMut, Response, BankMsg, Coin, WasmMsg};
use cosmwasm_std::WasmMsg::Execute as MsgExecuteContract;
use cosmwasm_std::Uint128;

use cw20::Cw20ExecuteMsg;

use crate::msg::Listing;
use crate::state::STATE;
use crate::error::ContractError;

use std::vec::Vec;

const FEE: f64 = 0.03;

fn delete<T>(vector: Vec<T>, target: &T) -> Vec<T> where T: PartialEq {
    let mut vec = vector;
    for (i, v) in vec.iter().enumerate() {
        if v == target {
            vec.remove(i);
            break;
        }
    }
    return vec
}

pub fn list(amount: u128, price: Uint128, sender: String, deps: DepsMut) -> Result<Response, ContractError> {
    let mut state = STATE.load(deps.storage)?;
    let mut listed = state.listed;
    listed.push(Listing {
        amount: amount,
        price: price,
        seller: sender,
    });
    listed.sort_by(|x, y| x.price.cmp(&y.price));
    state.listed = listed;
    STATE.save(deps.storage, &state)?;
    Ok(Response::default())
}

pub fn buy(amount: i128, sender: String, funds: Vec<Coin>, deps: DepsMut) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    let mut listed = state.listed;
    let mut suitable = Listing { amount: 0, price: Uint128::zero(), seller: "".to_string()};
    // make sure there even is a listing
    if listed.len() == 0 {
        return Err(ContractError::NotFound {});
    }
    let ammount_fixed = amount as u128; // convert to u128
    for listing in listed.clone() {
        // check if the amount is valid
        if listing.amount < ammount_fixed {
            continue;
        }
        // check if order is correctly priced
        if (listing.price.u128() * ammount_fixed) + (listing.price.u128() * FEE as u128) > funds[0].amount.u128()  {
            continue;
        }
        suitable = listing; // get the first suitable listing based on the requirements
    }

    if suitable == (Listing { amount: 0, price: Uint128::zero(), seller: "".to_string()}) {
        return Err(ContractError::UnknownError {});
    }

    suitable = suitable;

    if suitable.amount > ammount_fixed {
        listed = delete(listed, &suitable.clone()); // remove from the vec so it cant be accidentally replicated
        suitable.amount -= ammount_fixed;
        // listed.push(suitable);
        listed.sort_by(|x, y| x.price.cmp(&y.price)); // put everything back into order
    } else if suitable.amount == ammount_fixed {
        delete(listed, &suitable);
    } else if suitable.amount < ammount_fixed { // this should be unreachable, however we need to be 100% sure
        return Err(ContractError::InvalidAmount {});
    }

    let mut resp = Response::new();
    let transfer = Cw20ExecuteMsg::Transfer {
        recipient: sender,
        amount: Uint128::from(ammount_fixed),
    };
    let execute_msg = WasmMsg::Execute {
        contract_addr: state.denom,
        msg: to_binary(&transfer)?,
        funds: vec![],
    };
    resp = resp.add_message(execute_msg);
    resp = resp.add_message(BankMsg::Send {
        to_address: suitable.seller,
        amount: funds,
    });
    Ok(resp)
}

pub fn delist(sender: String, deps: DepsMut) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    let mut listed = state.listed;
    let mut suitable: Option<Listing> = None;
    // make sure there even is a listing
    if listed.len() == 0 {
        return Err(ContractError::NotFound {});
    }
    for (_index, listing) in listed.iter().enumerate() {
        // check if the sender is the owner
        if listing.seller != sender {
            continue;
        }
        suitable = Some(listing.clone()); // this is technically sloppy naming, but hey, itll be wasm anyway
    }

    if suitable.is_none() {
        return Err(ContractError::NotFound {});
    }

    let mut resp = Response::new();

    listed = delete(listed, &suitable.clone().unwrap()); // remove from the vec so it cant be accidentally replicated
    listed.sort_by(|x, y| x.price.cmp(&y.price)); // put everything back into order

    resp = resp.add_message(MsgExecuteContract {
        contract_addr: state.denom,
        msg: to_binary(&Cw20ExecuteMsg::Transfer {
            recipient: sender,
            amount: Uint128::from(suitable.unwrap().amount),
        })?,
        funds: vec![],
    });

    Ok(resp)
}

// pub fn liquidate_pool(sender: String, deps: DepsMut) -> Result<Response, ContractError> {
//     let s = STATE.load(deps.storage)?;
//     if &sender != &s.seller {
//         return Err(ContractError::Unauthorized {});
//     }
//     let mut resp = Response::new();
//     for listing in s.listed {
//         let transfer = Cw20ExecuteMsg::Transfer {
//             recipient: sender,
//             amount: Uint128::from(listing.amount),
//         };
//         let execute_msg = WasmMsg::Execute {
//             contract_addr: s.denom,
//             msg: to_binary(&transfer)?,
//             funds: vec![],
//         };
//         resp = resp.add_message(execute_msg);
//     }
//     Ok(resp)
// }