# Terp Account Billboards: A Billboard For Your Wallet

This implementation is a compatible instance of [sg-names](https://github.com/public-awesome/names) for Terp Network. To the stargaze contributors, thank you for setting the tone with these!
<!-- ##  [API Docs](./API.md) -->

| Contract | Description |
| --- | --- |
| [Account Marketplace](./contracts/terp721-account-marketplace/README.md) | The secondary marketplace for accounts. Accounts are automatically listed here once they are minted. |
| [Account Minter](./contracts/terp721-account-manifold/README.md) | Account minter is responsible for minting, validating, and updating accounts and their metadata. |
| [Terp721-Account](./contracts/terp721-account/README.md) | A cw721 contract with on-chain metadata for an account. |
| [Account Registry Middleware](./contracts/account-registry-middleware/README.md) | Recieves hooks and functions as admin middleware for abstract-account framework on-chain registry |
<!-- 
## Smart Accounts

| Contract | Description |
| --- | --- |
| [terp-ed25519](./contracts/smart-accounts/terp-ed25519/README.md) |   |
| [terp-eth](./contracts/smart-accounts/terp-eth/README.md) |  |
| [terp-irl](./contracts/smart-accounts/terp-irl/README.md) |   |
| [terp-passkey](./contracts/smart-accounts/terp-passkey/README.md) |   |
| [terp-wavs](./contracts/smart-accounts/terp-wavs/README.md) |   |
| [terp-zktls](./contracts/smart-accounts/terp-zktls/README.md) |   | -->

## Scripting Library

In this repo are [cw-orchestrator scripts](../../scripts/src/bin/manual_deploy.rs) that highlight this first step.

### Compile the contracts

```sh
just wasm
```

### Test the workspace

```sh
cargo test
# for code coverage reports
cargo coverage
```

### Cw-Orchestrator

To run the integration tests:

```sh
cd scripts/ && cargo test
```

### Deploy

To learn more about the deployment scripts, [check here](./scripts/README).

### Documentation

Checkout some documentation [here](./docs/00_disclaimer).

## DISCLAIMER

TERP-NETWORK CODE IS PROVIDED “AS IS”, AT YOUR OWN RISK, AND WITHOUT WARRANTIES OF ANY KIND. No developer or entity involved in creating or instantiating Terp Network smart contracts will be liable for any claims or damages whatsoever associated with your use, inability to use, or your interaction with other users of Terp Network, including any direct, indirect, incidental, special, exemplary, punitive or consequential damages, or loss of profits, cryptocurrencies, tokens, or anything else of value. Although Discover Decentralization DAO, and it's members configured existing code for the accounts, it does not own or control the Terp Network network.

