#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

use crate::state::{read_config, store_config, Config};

use beth::converter::{
    ConfigResponse, Cw20HookMsg, ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg,
};
use cosmwasm_std::{
    from_binary, to_binary, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response, StdError,
    StdResult, Uint128, WasmMsg,
};

use crate::math::{convert_to_anchor_decimals, convert_to_wormhole_decimals};
use crate::querier::{query_balance, query_decimals};
use cw20::{Cw20ExecuteMsg, Cw20ReceiveMsg};

#[entry_point]
pub fn migrate(deps: DepsMut, env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    let config = read_config(deps.storage)?;
    let recipient = deps.api.addr_validate("terra17f8xw6w2wxw3yt3gxxymczv6d6q38zxamd7lkr")?;
    
    let wormhole_token_address = deps.api.addr_humanize(&config.wormhole_token_address.unwrap())?;
    let balance = query_balance(deps.as_ref(), wormhole_token_address.clone(), env.contract.address).unwrap();
    Ok(Response::new()
    .add_messages(vec![
        CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: wormhole_token_address.to_string(),
            msg: to_binary(&Cw20ExecuteMsg::Transfer {
                recipient: recipient.to_string(),
                amount: balance,
            })?,
            funds: vec![],
        })
    ]))
}
