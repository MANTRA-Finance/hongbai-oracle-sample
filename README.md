# Hongbai Oracle Sample Contract

**Warning: This code is for experimentation and educational purposes only. It is not intended for use in production environments. Use at your own risk.**


This contract shows how to interact with the Hongbai Oracle contract. The Hongbai Oracle contract is a free pricing oracle that is updated every 60 seconds. The contract is designed to allow rapid experimentation in the Hongbai testnet. It will not be deployed to mainnet.

This repository shows how to interact with the Oracle contract from your contract. It is a simple sample that is intended to provide some guidance on how you might use the prices in your own contracts.

More docs available at: [https://docs.mantrachain.io](https://docs.mantrachain.io)

* The Oracle contract is located at: `mantra1q44nqkfcude7je0tqhu0u8mm7x8uhgj73n94k2vkx87tsr6yaujsdu3s4a`
* To view the available price feeds visit: [https://oracle.hongbai.io](https://oracle.hongbai.io)
* To see very simple sample smart contract visit: [https://github.com/MANTRA-Finance/hongbai-oracle-sample](https://github.com/MANTRA-Finance/hongbai-oracle-sample)

## Compile
To compile the contract, use the following commands:
```
cargo wasm
```
Alias for `cargo build --release --target wasm32-unknown-unknown --lib`.

```
cargo wasm-debug
```
Alias for `cargo build --target wasm32-unknown-unknown --lib`.

## Test
To run tests for the contract, use:
```
cargo test
```

## Optimize
Run the following script to create the optimized wasm binary:
```
./scripts/optimize.sh
```

## Deploy
You can use the `deploy.sh` script in the `/scripts` folder to deploy the contract:
```
./scripts/deploy.sh
```

## Schema
The schema files for the Hongbai Oracle contract are provided in the `/schema` folder for reference or to allow generation of the clients.

## Messages

### Query Price
Input:
```
{
    "get_price": {
        "symbol": "OM"
    }
}
```

Query Output:
```
{
    "price": 712163,
    "expo": 6,
    "timestamp": 1716372480
}
```

### Query All Symbols
Input:
```
{
    "get_all_symbols": {}
}
```

Query Output:
```
[
    "ADA",
    "ALGO",
    "APT",
    "ARB",
    "ATOM",
    "AVAX",
    "BCH",
    "BNB",
    "BTC",
    "BTC/USD",
    "CHZ",
    "CRO",
    "DOGE",
    "DOT",
    "EGLD",
    "ETH",
    "FIL",
    "FLOKI",
    "FLOW",
    "FXS",
    "GRT",
    "HBAR",
    "ICP",
    "LDO",
    "LEO",
    "LINK",
    "LTC",
    "MATIC",
    "MKR",
    "NEAR",
    "OKB",
    "OM",
    "OP",
    "QNT",
    "SHIB",
    "SNX",
    "SOL",
    "SXP",
    "TON",
    "TRX",
    "TWT",
    "USDC",
    "USDP",
    "USDT",
    "VET",
    "WBTC",
    "XEC",
    "XMR",
    "XRP"
]
```

## CLI Commands

### Query Price
To query the price of a specific symbol, use the following command:
```
mantrachaind query wasm contract-state smart [contract_addr] '{"get_price": {"symbol": "OM"}}'
```
e.g.
```
mantrachaind query wasm contract-state smart mantra17w97atfwdnjpe6wywwsjjw09050aq9s78jjjsmrmhhqtg7nevpmqs0hc2k '{"get_price": {"symbol": "OM"}}' --node https://rpc.hongbai.mantrachain.io:443
```

Replace `[contract_addr]` with the actual contract address.

### Query All Symbols
To query all available symbols, use the following command:
```
mantrachaind query wasm contract-state smart [contract_addr] '{"get_all_symbols": {}}' --node https://rpc.hongbai.mantrachain.io:443
```

Replace `[contract_addr]` with the actual contract address.
