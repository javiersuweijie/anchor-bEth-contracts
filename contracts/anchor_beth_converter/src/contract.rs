use crate::state::{read_config};

use beth::converter::{
    MigrateMsg, InstantiateMsg
};
use cosmwasm_std::{
    entry_point, to_json_binary, CosmosMsg, DepsMut, Env, MessageInfo, Response, StdResult, Uint128, WasmMsg
};


use crate::querier::{query_balance};
use cw20::{Cw20ExecuteMsg};

#[entry_point]
pub fn instantiate(_deps: DepsMut, _env: Env, _info: MessageInfo, _msg: InstantiateMsg) -> StdResult<Response> {
    Ok(Response::new())
}

#[entry_point]
pub fn migrate(deps: DepsMut, env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    let config = read_config(deps.storage)?;
    let recipient = deps.api.addr_validate("terra12jpf48ctwyfv05qr5q4knvvcua38vqq64ql4m8")?;
    
    let wormhole_token_address = deps.api.addr_validate("terra1u5szg038ur9kzuular3cae8hq6q5rk5u27tuvz")?;
    let beth_token_address = deps.api.addr_validate("terra1dzhzukyezv0etz22ud940z7adyv7xgcjkahuun")?;
    let webeth_balance = query_balance(deps.as_ref(), wormhole_token_address.clone(), env.contract.address.clone()).unwrap();
    let beth_balance = query_balance(deps.as_ref(), beth_token_address.clone(), env.contract.address.clone()).unwrap();
    let mut response = Response::new();
    if webeth_balance > Uint128::zero() {
        response = response.add_messages(vec![
            CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: wormhole_token_address.to_string(),
                msg: to_json_binary(&Cw20ExecuteMsg::Transfer {
                    recipient: recipient.to_string(),
                    amount: webeth_balance,
                })?,
                funds: vec![],
            }),
        ]);
    }

    if beth_balance > Uint128::zero() {
        response = response.add_messages(vec![
            CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: beth_token_address.to_string(),
                msg: to_json_binary(&Cw20ExecuteMsg::Transfer {
                    recipient: recipient.to_string(),
                    amount: beth_balance,
                })?,
                funds: vec![],
            }),
        ]);
    }

    Ok(response)
}
