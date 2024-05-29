use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::Item;

#[cw_serde]
pub struct Price {
    pub price: i64,
    pub expo: i32,
    pub timestamp: u64,
}

pub const ORACLE_ADDRESS: Item<Addr> = Item::new("oracle_address");
