#[cfg(test)]
mod tests {
    use std::marker::PhantomData;

    use cosmwasm_std::testing::MockQuerier;
    use cosmwasm_std::testing::{mock_env, mock_info, MockApi, MockStorage};
    use cosmwasm_std::{Addr, OwnedDeps};
    use hongbai_oracle::instantiate;
    use hongbai_oracle::msg::InstantiateMsg;
    use hongbai_oracle::state::ORACLE_ADDRESS;

    // Create mock dependencies with custom API
    pub fn mock_dependencies() -> OwnedDeps<MockStorage, MockApi, MockQuerier> {
        OwnedDeps {
            storage: MockStorage::new(),
            api: MockApi::default().with_prefix("mantra"),
            querier: MockQuerier::new(&[]),
            custom_query_type: PhantomData,
        }
    }

    #[test]
    fn test_instantiate() {
        let mut deps = mock_dependencies();
        let mock_contract_addr = deps.api.addr_make("hongbai-oracle");
        let msg = InstantiateMsg {
            oracle_address: mock_contract_addr.to_string(),
        };
        let info = mock_info("", &[]);
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(res.messages.len(), 0);

        let stored = ORACLE_ADDRESS.load(&deps.storage).unwrap();
        assert_eq!(stored, Addr::unchecked(mock_contract_addr.to_string()));
    }
}
