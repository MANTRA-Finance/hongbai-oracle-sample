use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    pub oracle_address: String,
}

#[cw_serde]
pub enum ExecMsg {}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(PriceResponse)]
    GetPrice { symbol: String },
    #[returns(Vec<String>)]
    GetAllSymbols {},
}

#[cw_serde]
pub struct PriceResponse {
    pub price: i64,
    pub expo: i32,
    pub timestamp: u64,
}
