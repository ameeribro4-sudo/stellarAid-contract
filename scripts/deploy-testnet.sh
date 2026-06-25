#!/usr/bin/env bash
set -euo pipefail

# This script deploys the factory and campaign contracts to the testnet.
# It's designed to be idempotent and saves the deployment information to a JSON file.

# --- Environment Setup ---
if [ -f ".env" ]; then
  set -o allexport
  source .env
  set +o allexport
fi

NETWORK="testnet"
RPC_URL="https://soroban-testnet.stellar.org:443"
PASSPHRASE="Test SDF Network ; September 2015"
DEPLOYMENTS_DIR="deployments"
DEPLOYMENT_FILE="${DEPLOYMENTS_DIR}/${NETWORK}.json"

# --- Build WASM ---
echo "Building WASM files..."
make build-wasm

# --- Paths ---
FACTORY_WASM_PATH="target/wasm32-unknown-unknown/release/factory.wasm"
CAMPAIGN_WASM_PATH="target/wasm32-unknown-unknown/release/campaign.wasm"

# --- Validate Prerequisites ---
if [ ! -f "$FACTORY_WASM_PATH" ]; then
  echo "❌ Factory WASM not found at $FACTORY_WASM_PATH"
  exit 1
fi

if [ ! -f "$CAMPAIGN_WASM_PATH" ]; then
  echo "❌ Campaign WASM not found at $CAMPAIGN_WASM_PATH"
  exit 1
fi

if [ -z "${SOROBAN_ADMIN_SECRET_KEY:-}" ]; then
  echo "❌ SOROBAN_ADMIN_SECRET_KEY is not set. Add it to .env or export it."
  exit 1
fi

# --- Idempotency Check ---
if [ -f "$DEPLOYMENT_FILE" ]; then
  echo "ℹ️  Deployment file already exists at $DEPLOYMENT_FILE. Skipping deployment."
  exit 0
fi

# --- Deploy ---
echo "🚀 Deploying to $NETWORK..."

# Install Campaign WASM
echo "Installing campaign WASM..."
CAMPAIGN_WASM_HASH=$(soroban contract install \
  --wasm "$CAMPAIGN_WASM_PATH" \
  --source "$SOROBAN_ADMIN_SECRET_KEY" \
  --rpc-url "$RPC_URL" \
  --network-passphrase "$PASSPHRASE")
echo "✅ Campaign WASM installed with hash: $CAMPAIGN_WASM_HASH"

# Deploy Factory Contract
echo "Deploying factory contract..."
FACTORY_CONTRACT_ID=$(soroban contract deploy \
  --wasm "$FACTORY_WASM_PATH" \
  --source "$SOROBAN_ADMIN_SECRET_KEY" \
  --rpc-url "$RPC_URL" \
  --network-passphrase "$PASSPHRASE")
echo "✅ Factory contract deployed with ID: $FACTORY_CONTRACT_ID"

# --- Initialize Factory ---
echo "Initializing factory contract..."
soroban contract invoke \
  --id "$FACTORY_CONTRACT_ID" \
  --source "$SOROBAN_ADMIN_SECRET_KEY" \
  --rpc-url "$RPC_URL" \
  --network-passphrase "$PASSPHRASE" \
  -- \
  initialize \
  --admin "$SOROBAN_ADMIN_SECRET_KEY" \
  --campaign_wasm_hash "$CAMPAIGN_WASM_HASH"
echo "✅ Factory contract initialized."

# --- Persist Deployment Record ---
mkdir -p "$DEPLOYMENTS_DIR"
TIMESTAMP=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
cat > "$DEPLOYMENT_FILE" <<EOF
{
  "network": "$NETWORK",
  "factory_contract_id": "$FACTORY_CONTRACT_ID",
  "campaign_wasm_hash": "$CAMPAIGN_WASM_HASH",
  "rpc_url": "$RPC_URL",
  "deployed_at": "$TIMESTAMP"
}
EOF

echo "💾 Deployment record saved to $DEPLOYMENT_FILE"