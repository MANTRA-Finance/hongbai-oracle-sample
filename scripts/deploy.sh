#!/bin/bash

# Exit script on any error
set -e

# Configuration
CLIPATH="${HOME}/repos/mantrachain/build"
CHAIN_ID="mantra-hongbai-1"
NODE="https://rpc.hongbai.mantrachain.io:443"
FROM="oracle" # Your account name
GAS="1600000"
GAS_ADJUSTMENT="2"
GAS_PRICES="0.01uom"
LABEL="Hongbai Sample"
INIT_MSG='{"oracle_address":"mantra1q44nqkfcude7je0tqhu0u8mm7x8uhgj73n94k2vkx87tsr6yaujsdu3s4a"}'
CONTRACT=artifacts/hongbai_oracle_sample.wasm

echo "Ensure you have run optimize.sh before running this script"

# Upload the wasm binary
echo "Uploading the wasm binary..."
TX_UPLOAD_HASH=$(${CLIPATH}/mantrachaind tx wasm store $CONTRACT --from $FROM --gas $GAS --gas-adjustment $GAS_ADJUSTMENT --gas-prices $GAS_PRICES --node $NODE --chain-id $CHAIN_ID --broadcast-mode sync -y -o json)
TX_UPLOAD_HASH=$(echo $TX_UPLOAD_HASH | jq -r '.txhash')

# Verify transaction hash is not empty
if [ -z "$TX_UPLOAD_HASH" ]; then
  echo "Error: Transaction hash not found in the upload response."
  exit 1
fi

echo "Wasm binary uploaded. Transaction hash: $TX_UPLOAD_HASH"

# Query the transaction to get the code ID
echo "Querying the transaction to get the code ID..."
sleep 7
TX_UPLOAD_RESULT=$(${CLIPATH}/mantrachaind query tx $TX_UPLOAD_HASH --node $NODE --chain-id $CHAIN_ID -o json)
CODE_ID=$(echo $TX_UPLOAD_RESULT | jq -r '.logs[0].events[] | select(.type=="store_code") | .attributes[] | select(.key=="code_id") | .value')

if [ -z "$CODE_ID" ]; then
  echo "Error: Code ID not found in the transaction result."
  exit 1
fi

echo "Wasm binary uploaded with Code ID: $CODE_ID"

# Instantiate the contract
echo "Instantiating the smart contract..."
TX_INSTANTIATE_HASH=$(${CLIPATH}/mantrachaind tx wasm instantiate $CODE_ID "$INIT_MSG" --from $FROM --no-admin --label "$LABEL" --gas $GAS --gas-adjustment $GAS_ADJUSTMENT --gas-prices $GAS_PRICES --node $NODE --chain-id $CHAIN_ID --broadcast-mode sync -y -o json)
TX_INSTANTIATE_HASH=$(echo $TX_INSTANTIATE_HASH | jq -r '.txhash')
sleep 7
TX_INSTANTIATE=$(${CLIPATH}/mantrachaind query tx $TX_INSTANTIATE_HASH --node $NODE --chain-id $CHAIN_ID -o json)

CONTRACT_ADDRESS=$(echo $TX_INSTANTIATE | jq -r '.events[] | select(.type == "instantiate") | .attributes[] | select(.key == "_contract_address").value')

if [ -z "$CONTRACT_ADDRESS" ]; then
  echo "Error: Contract address not found in the transaction response."
  exit 1
fi

echo "Smart contract instantiated with Contract Address: $CONTRACT_ADDRESS"

# Print a nice message
echo "Deployment Summary:"
echo "===================="
echo "Code ID: $CODE_ID"
echo "Contract Address: $CONTRACT_ADDRESS"


