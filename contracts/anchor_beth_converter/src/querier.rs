use cosmwasm_std::{to_binary, Addr, Deps, QueryRequest, StdResult, WasmQuery, Uint128};
use cw20::{Cw20QueryMsg, TokenInfoResponse, BalanceResponse};

pub fn query_decimals(deps: Deps, contract_addr: Addr) -> StdResult<u8> {
    // load price form the oracle
    let token_info: TokenInfoResponse =
        deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: contract_addr.to_string(),
            msg: to_binary(&Cw20QueryMsg::TokenInfo {})?,
        }))?;

    Ok(token_info.decimals)
}

pub fn query_balance(deps: Deps, contract_addr: Addr, address: Addr) -> StdResult<Uint128> {
    let balance: BalanceResponse = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: contract_addr.to_string(),
        msg: to_binary(&Cw20QueryMsg::Balance { address: address.to_string() })?,
    }))?;

    Ok(balance.balance)
}
