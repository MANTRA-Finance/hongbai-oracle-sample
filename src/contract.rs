use crate::msg::{ExecMsg, InstantiateMsg, PriceResponse, QueryMsg};
use crate::state::ORACLE_ADDRESS;
use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, QueryRequest, Response,
    StdResult, WasmQuery,
};
use serde::{Deserialize, Serialize};

/// Entry point for contract instantiation.
/// Initializes the contract with the given oracle address.
#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    // Validate the oracle address provided in the instantiation message.
    let oracle_address = deps.api.addr_validate(&msg.oracle_address)?;
    // Save the validated oracle address to the contract's state.
    ORACLE_ADDRESS.save(deps.storage, &oracle_address)?;

    // Return a response indicating successful instantiation.
    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("type", "Hongbai Oracle User"))
}

/// Entry point for contract execution.
/// Currently not implemented.
#[entry_point]
pub fn execute(_deps: DepsMut, _env: Env, _info: MessageInfo, msg: ExecMsg) -> StdResult<Response> {
    match msg {}
}

/// Entry point for contract queries.
/// Handles queries for getting the price of a symbol and getting all available symbols.
#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        // Handle query to get the price of a specific symbol.
        QueryMsg::GetPrice { symbol } => to_json_binary(&query_price(deps, symbol)?),
        // Handle query to get all available symbols.
        QueryMsg::GetAllSymbols {} => to_json_binary(&query_all_symbols(deps)?),
    }
}

/// Helper function to query the price of a specific symbol from the oracle.
/// Sends a smart contract query to the oracle contract.
fn query_price(deps: Deps, symbol: String) -> StdResult<PriceResponse> {
    // Load the oracle address from the contract's state.
    let oracle_address = ORACLE_ADDRESS.load(deps.storage)?;

    // Create the query message for getting the price.
    let msg = QueryPrice {
        get_price: GetPrice { symbol },
    };

    // Create the smart contract query request.
    let request = QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: oracle_address.to_string(),
        msg: to_json_binary(&msg)?,
    });

    // Execute the query and parse the response.
    let res: PriceResponse = deps.querier.query(&request)?;

    Ok(res)
}

/// Helper function to query all available symbols from the oracle.
/// Sends a smart contract query to the oracle contract.
fn query_all_symbols(deps: Deps) -> StdResult<Vec<String>> {
    // Load the oracle address from the contract's state.
    let oracle_address = ORACLE_ADDRESS.load(deps.storage)?;

    // Create the query message for getting all symbols.
    let msg = QueryAllSymbols {
        get_all_symbols: GetAllSymbols {},
    };

    // Create the smart contract query request.
    let request = QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: oracle_address.to_string(),
        msg: to_json_binary(&msg)?,
    });

    // Execute the query and parse the response.
    let res: Vec<String> = deps.querier.query(&request)?;
    Ok(res)
}

/// Structure for querying the price of a symbol.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct QueryPrice {
    pub get_price: GetPrice,
}

/// Structure representing the price query request.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct GetPrice {
    pub symbol: String,
}

/// Structure for querying all available symbols.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct QueryAllSymbols {
    pub get_all_symbols: GetAllSymbols,
}

/// Empty structure representing the request for all symbols.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct GetAllSymbols {}
