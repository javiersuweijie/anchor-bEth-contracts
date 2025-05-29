use cosmwasm_std::{Coin, Decimal, Deps, StdResult, Uint128};

static DECIMAL_FRACTION: Uint128 = Uint128::new(1_000_000_000_000_000_000u128);

pub fn compute_tax(deps: Deps, coin: &Coin) -> StdResult<Uint128> {
    let tax_rate: Decimal = Decimal::zero();
    let tax_cap: Uint128 = Uint128::new(1000000000000000000u128);
    Ok(std::cmp::min(
        coin.amount.checked_sub(coin.amount.multiply_ratio(
            DECIMAL_FRACTION,
            DECIMAL_FRACTION * tax_rate + DECIMAL_FRACTION,
        ))?,
        tax_cap,
    ))
}

pub fn deduct_tax(deps: Deps, coin: Coin) -> StdResult<Coin> {
    let tax_amount = compute_tax(deps, &coin)?;
    Ok(Coin {
        denom: coin.denom,
        amount: coin.amount.checked_sub(tax_amount)?,
    })
}
